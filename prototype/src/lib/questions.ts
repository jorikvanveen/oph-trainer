export const topics = [
  {
    id: 'sources',
    name: 'Music sources',
    description: 'Analog, digital, and where music begins.',
    icon: 'disc',
    color: 'sage',
    source: 'analog-and-digital-music-sources.php',
  },
  {
    id: 'formats',
    name: 'Audio formats',
    description: 'Lossless, lossy, and what gets left behind.',
    icon: 'audio',
    color: 'blue',
    source: 'audio-formats.php',
  },
  {
    id: 'mp3',
    name: 'MP3',
    description: 'Encoders, bitrates, and quality presets.',
    icon: 'sliders',
    color: 'sand',
    source: 'mp3.php',
  },
  {
    id: 'transcodes',
    name: 'Transcodes',
    description: 'Know a good conversion from a bad one.',
    icon: 'repeat',
    color: 'rose',
    source: 'transcodes.php',
  },
  {
    id: 'torrenting',
    name: 'Torrenting',
    description: 'Peers, seeding, and keeping a healthy ratio.',
    icon: 'network',
    color: 'lavender',
    source: 'torrenting.php',
  },
  {
    id: 'spectral',
    name: 'Spectral analysis',
    description: 'Read the frequencies. Spot the cut-off.',
    icon: 'chart',
    color: 'sage',
    source: 'spectral-analysis.php',
  },
  {
    id: 'ripping',
    name: 'CD burning & ripping',
    description: 'Understand rips, logs, and cue sheets.',
    icon: 'disc',
    color: 'sand',
    source: 'cd-burning-and-cd-ripping.php',
  },
  {
    id: 'dupes',
    name: 'Dupes & trumps',
    description: 'Editions, duplicates, and better uploads.',
    icon: 'layers',
    color: 'blue',
    source: 'dupes-and-trumps.php',
  },
  {
    id: 'rules',
    name: 'Orpheus rules',
    description: 'The rules that keep the community running.',
    icon: 'shield',
    color: 'rose',
    source: 'orpheus-rules.php',
  },
] as const

export type TopicId = (typeof topics)[number]['id']
export type Question = {
  id: string
  topic: TopicId
  prompt: string
  type: 'short' | 'explanation'
  reference: string
  source: string
  accepted?: string[]
  image?: { url: string; alt: string }
  excerpt?: string
}

export const syllabusUrl = 'https://interview.orpheus.network/'
const source = (page: string) => syllabusUrl + page

// A small syllabus-derived sample for exploring the UI, not the full question bank.
export const questions: Question[] = [
  {
    id: 'sources-converter',
    topic: 'sources',
    type: 'short',
    prompt:
      'What kind of converter turns an analog music source into a digital file?',
    accepted: [
      'analog to digital converter',
      'analog-to-digital converter',
      'adc',
      'a/d converter',
    ],
    reference:
      'An analog-to-digital converter, such as a sound card, turns the physical signal from an analog medium into digital data.',
    source: source('analog-and-digital-music-sources.php'),
  },
  {
    id: 'sources-vinyl',
    topic: 'sources',
    type: 'short',
    prompt: 'Is a vinyl record an analog or a digital music source?',
    accepted: ['analog', 'analogue', 'analog source', 'analogue source'],
    reference:
      'Vinyl is an analog source. The grooves physically represent sound; an analog-to-digital converter is needed to make a digital copy.',
    source: source('analog-and-digital-music-sources.php'),
  },
  {
    id: 'formats-flac',
    topic: 'formats',
    type: 'short',
    prompt: 'What does FLAC stand for?',
    accepted: ['free lossless audio codec'],
    reference:
      'FLAC stands for Free Lossless Audio Codec. It is a compressed lossless format that retains all the original audio data.',
    source: source('audio-formats.php'),
  },
  {
    id: 'formats-lossless',
    topic: 'formats',
    type: 'explanation',
    prompt:
      'How can compressed lossless audio use less space without losing audio information?',
    reference:
      'Compressed lossless formats retain all the original recorded data while representing it more efficiently. They give silence very few bits and compress sound. Decoding restores the original audio data; unlike lossy compression, it does not discard information.',
    source: source('audio-formats.php'),
  },
  {
    id: 'mp3-vbr',
    topic: 'mp3',
    type: 'short',
    prompt: 'What does VBR stand for?',
    accepted: ['variable bitrate', 'variable bit rate'],
    reference:
      'VBR stands for variable bitrate. The encoder targets a chosen quality while letting the bitrate change with the complexity of the audio.',
    source: source('mp3.php'),
  },
  {
    id: 'mp3-lame',
    topic: 'mp3',
    type: 'short',
    prompt: 'Which MP3 encoder does the official guide recommend?',
    accepted: ['lame', 'lame encoder'],
    reference:
      'The guide recommends LAME because it is open source, customizable, and produces high-quality MP3 files.',
    source: source('mp3.php'),
  },
  {
    id: 'transcodes-lossy-flac',
    topic: 'transcodes',
    type: 'explanation',
    prompt:
      'Why does converting an MP3 to FLAC not make it a lossless original?',
    reference:
      'The MP3 encoding already discarded audio data. FLAC can preserve what remains but cannot recover the missing data. This is a lossy-to-lossless transcode, which is prohibited on Orpheus.',
    source: source('transcodes.php'),
  },
  {
    id: 'transcodes-lossless',
    topic: 'transcodes',
    type: 'explanation',
    prompt:
      'Is converting a lossless WAV file to FLAC a good transcode? Explain why.',
    reference:
      'Yes. Lossless-to-lossless conversion preserves the original audio data, so converting a lossless WAV file to FLAC does not reduce its quality.',
    source: source('transcodes.php'),
  },
  {
    id: 'torrenting-seeding',
    topic: 'torrenting',
    type: 'short',
    prompt:
      'What is it called when you have finished downloading and keep the data available to upload to other peers?',
    accepted: ['seeding', 'seed'],
    reference:
      'This is seeding. Upload credit is earned when another peer actually downloads data from you, not merely because a torrent is available.',
    source: source('torrenting.php'),
  },
  {
    id: 'torrenting-ratio',
    topic: 'torrenting',
    type: 'short',
    prompt: 'You have uploaded 8 GB and downloaded 4 GB. What is your ratio?',
    accepted: ['2', '2.0', '2.00', '2:1'],
    reference:
      'Your ratio is 2.0. Ratio is uploaded data divided by downloaded data, so 8 divided by 4 equals 2.',
    source: source('torrenting.php'),
  },
  {
    id: 'spectral-128',
    topic: 'spectral',
    type: 'explanation',
    prompt:
      'Describe the frequency cut-off in this spectral. Which MP3 bitrate does the guide associate with it?',
    image: {
      url: source('Guide-MP3-128-CBR.jpg'),
      alt: 'Spectral diagram from the official guide, with frequency in kHz vertically and time horizontally.',
    },
    reference:
      'The visible cut-off is at about 16 kHz. The guide associates this spectral with MP3 128 kbps CBR. Spectral patterns provide clues about the encoding and should be interpreted in context.',
    source: source('spectral-analysis.php'),
  },
  {
    id: 'spectral-transcode',
    topic: 'spectral',
    type: 'explanation',
    prompt:
      'This file is labeled FLAC. What in its spectral suggests that it came from a lossy source?',
    image: {
      url: source('Guide-MP3-192-to-FLAC.jpg'),
      alt: 'Spectral diagram of a file labeled FLAC, showing an abrupt upper-frequency cut-off.',
    },
    reference:
      "The sharp cut-off around 19 kHz resembles the guide's MP3 192 kbps CBR example rather than its typical lossless example. Converting MP3 to FLAC does not restore discarded frequencies. The guide identifies this example as a lossy-to-lossless transcode.",
    source: source('spectral-analysis.php'),
  },
  {
    id: 'ripping-log',
    topic: 'ripping',
    type: 'short',
    prompt:
      'Which file type records the CD-ripping process and any errors that occurred?',
    accepted: ['log', '.log', 'log file', 'a log file'],
    reference:
      'A log file, with the .log extension, records the ripping process and its errors. You must never modify a ripping log.',
    source: source('cd-burning-and-cd-ripping.php'),
  },
  {
    id: 'ripping-cue',
    topic: 'ripping',
    type: 'explanation',
    prompt: 'What kind of file is this excerpt from, and what is its purpose?',
    excerpt:
      'FILE "01 The Carnival of the Animals - Introduction and Royal March of the Lion.wav" WAVE\n  TRACK 01 AUDIO\n    TITLE "The Carnival of the Animals - Introduction and Royal March of the Lion"\n    PREGAP 00:00:32\n    INDEX 01 00:00:00',
    reference:
      "This is a cue sheet. It describes a CD's structure, including tracks and timing, and acts as a table of contents so a CD can be burned to match the original.",
    source: source('cue-file.php'),
  },
  {
    id: 'dupes-definition',
    topic: 'dupes',
    type: 'short',
    prompt:
      'What is a new torrent called when it is the same edition and quality as an existing torrent?',
    accepted: ['dupe', 'a dupe', 'duplicate', 'a duplicate'],
    reference:
      'It is a dupe, or duplicate. Torrents of the same edition with the same format, bitrate, and comparable or identical sampling rates are duplicates.',
    source: source('dupes-and-trumps.php'),
  },
  {
    id: 'dupes-editions',
    topic: 'dupes',
    type: 'explanation',
    prompt:
      'Do different catalog numbers alone make two CDs distinct editions on Orpheus?',
    reference:
      'No. A distinct edition requires significantly and meaningfully different audio content. Catalog numbers, release years, labels, countries, or packaging differences alone do not establish a new edition.',
    source: source('dupes-and-trumps.php'),
  },
  {
    id: 'rules-accounts',
    topic: 'rules',
    type: 'short',
    prompt:
      'How many Orpheus accounts is one person allowed to have over their lifetime?',
    accepted: ['one', '1', 'one account', '1 account'],
    reference:
      'One account per person, per lifetime. If an account is disabled, contact staff through the designated IRC channel rather than creating another account.',
    source: source('orpheus-rules.php'),
  },
  {
    id: 'rules-passkey',
    topic: 'rules',
    type: 'explanation',
    prompt: 'Why must you not share your Orpheus .torrent files or passkey?',
    reference:
      'An Orpheus .torrent file contains an announce URL with your personal passkey. The passkey lets clients report statistics to the tracker. The golden rules prohibit sharing either the file or your passkey.',
    source: source('orpheus-rules.php'),
  },
]
