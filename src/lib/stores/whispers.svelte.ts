// Soft Landings — gentle toast notifications
// Errors are called "whispers from the void" in the Noctuary

interface Whisper {
  id: number;
  message: string;
  kind: 'gentle' | 'warn' | 'glow';
  fading: boolean;
}

let nextId = 0;
let whispers: Whisper[] = $state([]);

const GENTLE_PHRASES = [
  'The void stirs but does not answer',
  'A thread has come loose in the weave',
  'The stars flicker — something went astray',
  'The ritual faltered, but the circle holds',
  'A shadow passed over the connection',
  'The ley lines wavered for a moment',
];

function randomGentle(): string {
  return GENTLE_PHRASES[Math.floor(Math.random() * GENTLE_PHRASES.length)];
}

export function whisper(message: string, kind: Whisper['kind'] = 'gentle') {
  const id = nextId++;
  whispers.push({ id, message, kind, fading: false });

  // Begin fade after 3s
  setTimeout(() => {
    const w = whispers.find((w) => w.id === id);
    if (w) w.fading = true;
  }, 3000);

  // Remove after fade completes (0.6s)
  setTimeout(() => {
    whispers = whispers.filter((w) => w.id !== id);
  }, 3600);
}

/** Gentle error with optional cosmic context */
export function softError(detail?: string) {
  const cosmic = randomGentle();
  const msg = detail ? `${cosmic} — ${detail}` : cosmic;
  whisper(msg, 'gentle');
}

/** Positive feedback glow */
export function softGlow(message: string) {
  whisper(message, 'glow');
}

export function getWhispers(): Whisper[] {
  return whispers;
}
