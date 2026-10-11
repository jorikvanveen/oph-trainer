import { createContext } from 'svelte';
import { api, errorMessage } from '#lib/api.ts';
import type { QuestionCatalogDto } from '#lib/dto/QuestionCatalogDto.ts';
import type { QuizSelectionResponseDto } from '#lib/dto/QuizSelectionResponseDto.ts';
import type { QuizSelectionRequestDto } from '#lib/dto/QuizSelectionRequestDto.ts';
import type { QuestionOutcomeDto } from '#lib/dto/QuestionOutcomeDto.ts';
import type { CommitQuizDto } from '#lib/dto/CommitQuizDto.ts';
import type { ProgressDto } from '#lib/dto/ProgressDto.ts';
import type { AccountDto } from '#lib/dto/AccountDto.ts';

const keyStorage = 'oph-trainer.account-key';

export class AppState {
  catalog = $state<QuestionCatalogDto | null>(null);
  accountKey = $state('');
  accountOpen = $state(false);
  progress = $state<ProgressDto | null>(null);
  progressError = $state('');
  error = $state('');
  busy = $state(true);
  selected = $state<string[]>([]);
  quiz = $state<QuizSelectionResponseDto | null>(null);
  outcomes = $state<QuestionOutcomeDto[]>([]);
  saved = $state(false);

  get unfinished() {
    return this.quiz !== null && !this.saved;
  }

  async initialize() {
    this.busy = true;
    this.error = '';
    try {
      this.accountKey = localStorage.getItem(keyStorage) ?? '';
      if (!this.accountKey) {
        const account = await api<AccountDto>('/accounts', '', 'POST');
        this.retainKey(account.account_key);
      }
      this.catalog = await api<QuestionCatalogDto>('/catalog');
      this.selected = this.catalog.topics.map((topic) => topic.topic_id);
      await this.loadProgress();
    } catch (cause) {
      this.error = errorMessage(cause);
    } finally {
      this.busy = false;
    }
  }

  retainKey(key: string) {
    this.accountKey = key;
    localStorage.setItem(keyStorage, key);
  }

  async loadProgress() {
    this.progressError = '';
    try {
      this.progress = await api<ProgressDto>('/progress', this.accountKey);
    } catch (cause) {
      this.progress = null;
      this.progressError = errorMessage(cause);
    }
  }

  async refreshProgress() {
    this.busy = true;
    await this.loadProgress();
    this.busy = false;
  }

  discard() {
    this.quiz = null;
    this.outcomes = [];
    this.saved = false;
    this.error = '';
  }

  async startQuiz(): Promise<boolean> {
    if (this.busy || !this.catalog || this.selected.length === 0) return false;
    this.busy = true;
    this.error = '';
    try {
      const request: QuizSelectionRequestDto =
        this.selected.length === this.catalog.topics.length
          ? {}
          : { topic_ids: this.selected };
      const batch = await api<QuizSelectionResponseDto>(
        '/quizzes',
        this.accountKey,
        'POST',
        request
      );
      this.quiz = batch;
      this.outcomes = [];
      this.saved = false;
      await this.loadProgress();
      return true;
    } catch (cause) {
      this.error = errorMessage(cause);
      return false;
    } finally {
      this.busy = false;
    }
  }

  async saveQuiz() {
    if (
      this.busy ||
      !this.quiz ||
      this.saved ||
      this.outcomes.length !== this.quiz.questions.length
    )
      return;
    this.busy = true;
    this.error = '';
    try {
      const completed: CommitQuizDto = {
        quiz_id: this.quiz.quiz_id,
        question_ids: this.quiz.questions.map(
          (question) => question.question_id
        ),
        outcomes: this.outcomes
      };
      await api<void>('/progress/quizzes', this.accountKey, 'POST', completed);
      this.saved = true;
      await this.loadProgress();
    } catch (cause) {
      this.error = `${errorMessage(cause)} Save was not confirmed. Retry saving before leaving this page.`;
    } finally {
      this.busy = false;
    }
  }

  async switchAccount(key: string): Promise<boolean> {
    const candidate = key.trim();
    if (this.busy || !candidate) return false;
    this.busy = true;
    this.error = '';
    try {
      const progress = await api<ProgressDto>('/progress', candidate);
      this.discard();
      this.retainKey(candidate);
      this.progress = progress;
      this.progressError = '';
      this.accountOpen = false;
      return true;
    } catch (cause) {
      this.error = errorMessage(cause);
      return false;
    } finally {
      this.busy = false;
    }
  }
}

export const [getAppState, setAppState] = createContext<AppState>();
