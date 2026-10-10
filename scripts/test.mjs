import { spawn } from 'node:child_process';
import { existsSync } from 'node:fs';
import { cp, mkdir, mkdtemp, rm } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { basename, dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const frontend = join(root, 'frontend');
const target = join(root, 'backend', 'target');
const cli = join(frontend, 'node_modules', '@playwright', 'test', 'cli.js');
const seconds = Number(process.env.OPH_TEST_TIMEOUT_SECONDS ?? 600);
if (!Number.isInteger(seconds) || seconds < 1 || seconds > 3600) {
  console.error('OPH_TEST_TIMEOUT_SECONDS must be an integer between 1 and 3600.');
  process.exit(1);
}
if (!existsSync(cli)) {
  console.error('Install dependencies first with npm ci --prefix frontend.');
  process.exit(1);
}

const controller = new AbortController();
const children = new Set();
let exitCode = 1;
let temporary;
const deadline = setTimeout(() => {
  controller.abort(new Error(`Test command exceeded its ${seconds}s deadline.`));
}, seconds * 1000);
for (const [signal, code] of [['SIGINT', 130], ['SIGTERM', 143]]) {
  process.on(signal, () => {
    exitCode = code;
    controller.abort(new Error(`Test command interrupted by ${signal}.`));
  });
}

async function bounded(promise) {
  controller.signal.throwIfAborted();
  let onAbort;
  const interrupted = new Promise((_, reject) => {
    onAbort = () => reject(controller.signal.reason);
    controller.signal.addEventListener('abort', onAbort, { once: true });
  });
  try {
    return await Promise.race([promise, interrupted]);
  } finally {
    controller.signal.removeEventListener('abort', onAbort);
  }
}

function start(command, args, {
  cwd = root, env, capture = false, shutdownSignal = 'SIGTERM', shutdownTimeout = 2000
} = {}) {
  controller.signal.throwIfAborted();
  // Each command gets a process group so cleanup also reaches its descendants.
  const child = spawn(command, args, {
    cwd,
    env,
    detached: true,
    stdio: ['ignore', capture ? 'pipe' : 'inherit', capture ? 'pipe' : 'inherit']
  });
  const task = { child, output: '', finished: null, shutdownSignal, shutdownTimeout };
  task.finished = new Promise((resolve) => {
    child.once('error', (error) => resolve({ error }));
    child.once('close', (code, signal) => resolve({ code, signal }));
  });
  if (capture) {
    for (const stream of [child.stdout, child.stderr]) {
      stream.on('data', (chunk) => {
        process.stdout.write(chunk);
        task.output = (task.output + chunk.toString()).slice(-16_384);
      });
    }
  }
  children.add(task);
  return task;
}

function killGroup(task, signal) {
  if (!task.child.pid) return;
  try {
    process.kill(-task.child.pid, signal);
  } catch (error) {
    if (error.code !== 'ESRCH') throw error;
  }
}

async function stop(task) {
  killGroup(task, task.shutdownSignal);
  let timer;
  try {
    await Promise.race([
      task.finished,
      new Promise((resolve) => { timer = setTimeout(resolve, task.shutdownTimeout); })
    ]);
  } finally {
    clearTimeout(timer);
    // The group leader can exit before Chromium or another descendant does.
    killGroup(task, 'SIGKILL');
    await task.finished;
    children.delete(task);
  }
}

function failed(label, result) {
  return new Error(`${label} failed: ${result.error?.message ?? result.signal ?? `exit ${result.code}`}.`);
}

async function run(command, args, options, label) {
  const task = start(command, args, options);
  const result = await bounded(task.finished);
  await stop(task);
  if (result.code !== 0) throw failed(label, result);
}

async function serverOrigin(server) {
  let poll;
  let timer;
  const listening = new Promise((resolve, reject) => {
    poll = setInterval(() => {
      const log = server.output.replace(/\x1b\[[0-9;]*m/g, '');
      const address = log.match(/Listening[^\n]*address=127\.0\.0\.1:(\d+)/);
      if (address) resolve(`http://127.0.0.1:${address[1]}`);
    }, 50);
    timer = setTimeout(() => reject(new Error('Axum did not start within 30s.')), 30_000);
  });
  try {
    return await bounded(Promise.race([
      listening,
      server.finished.then((result) => { throw failed('Axum startup', result); })
    ]));
  } finally {
    clearInterval(poll);
    clearTimeout(timer);
  }
}

try {
  temporary = await mkdtemp(join(tmpdir(), 'oph-trainer-test-'));
  console.log(`Test workspace: ${temporary}`);
  await run('cargo', ['test', '--locked', '--workspace'], {
    cwd: join(root, 'backend'),
    env: { ...process.env, CARGO_TARGET_DIR: target }
  }, 'Rust tests');
  await run('bash', [join(root, 'scripts', 'build.sh')], {
    env: { ...process.env, CARGO_TARGET_DIR: target }
  }, 'Production build');
  await cp(join(frontend, 'build'), join(temporary, 'static'), { recursive: true });

  // Do not pass operator configuration or provider credentials to the service.
  const runtimeEnv = Object.fromEntries(
    ['PATH', 'HOME', 'LD_LIBRARY_PATH', 'FONTCONFIG_FILE', 'FONTCONFIG_PATH',
      'DISPLAY', 'WAYLAND_DISPLAY', 'XDG_RUNTIME_DIR', 'XDG_CACHE_HOME',
      'PLAYWRIGHT_BROWSERS_PATH', 'CI', 'CHROMIUM_PATH']
      .filter((name) => process.env[name] !== undefined)
      .map((name) => [name, process.env[name]])
  );
  runtimeEnv.TMPDIR = temporary;
  runtimeEnv.XDG_CONFIG_HOME = join(temporary, 'config');
  const browserCache = join(temporary, 'cache');
  await Promise.all([
    mkdir(runtimeEnv.XDG_CONFIG_HOME),
    mkdir(browserCache)
  ]);
  const server = start(join(target, 'release', 'oph-trainer-backend'), [], {
    cwd: temporary,
    capture: true,
    env: {
      ...runtimeEnv,
      OPH_BIND_ADDRESS: '127.0.0.1:0',
      OPH_DATABASE_URL: `sqlite://${join(temporary, 'test.db')}?mode=rwc`,
      OPH_STATIC_DIR: join(temporary, 'static'),
      RUST_LOG: 'info'
    }
  });
  const origin = await serverOrigin(server);
  const response = await fetch(origin, {
    signal: AbortSignal.any([controller.signal, AbortSignal.timeout(5000)])
  });
  if (!response.ok) throw new Error(`Static frontend readiness returned HTTP ${response.status}.`);
  await response.arrayBuffer();
  console.log(`Testing production app at ${origin}`);

  const tests = start(process.execPath, [cli, 'test', ...process.argv.slice(2)], {
    cwd: frontend,
    // Playwright handles SIGINT by closing its workers and detached browsers.
    shutdownSignal: 'SIGINT',
    shutdownTimeout: 10_000,
    env: {
      ...runtimeEnv,
      OPH_TEST_BASE_URL: origin,
      OPH_TEST_OUTPUT_DIR: join(frontend, 'test-results', basename(temporary)),
      OPH_TEST_BROWSER_CACHE_DIR: browserCache
    }
  });
  const result = await bounded(Promise.race([
    tests.finished,
    server.finished.then((result) => { throw failed('Axum during tests', result); })
  ]));
  if (result.code !== 0) throw failed('Playwright', result);
  exitCode = 0;
} catch (error) {
  console.error(error.message);
} finally {
  clearTimeout(deadline);
  await Promise.all([...children].map(stop));
  if (temporary) await rm(temporary, { recursive: true, force: true });
}
process.exitCode = exitCode;
