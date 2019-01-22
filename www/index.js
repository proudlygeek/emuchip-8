import { memory } from './emuchip_8_bg';
import { Emulator } from './emuchip_8';

const HEIGHT = 32;
const WIDTH = 64;
const loadGame = document.querySelector('#load-game');
const canvas = document.querySelector('#screen');
const ctx = canvas.getContext('2d');

const keyboardMapping = {
  '1': 0x1,
  '2': 0x2,
  '3': 0x3,
  '4': 0xc,
  q: 0x4,
  w: 0x5,
  e: 0x6,
  r: 0xd,
  a: 0x7,
  s: 0x8,
  d: 0x9,
  f: 0xe,
  z: 0xa,
  x: 0x0,
  c: 0xb,
  v: 0xf
};

const gamesList = [
  '15PUZZLE',
  'BLINKY',
  'BLITZ',
  'BRIX',
  'CONNECT4',
  'GUESS',
  'HIDDEN',
  'INVADERS',
  'KALEID',
  'MAZE',
  'MERLIN',
  'MISSILE',
  'PONG',
  'PONG2',
  'PUZZLE',
  'SYZYGY',
  'TANK',
  'TETRIS',
  'TICTAC',
  'UFO',
  'VBRIX',
  'VERS',
  'WIPEOFF'
];

const emu = Emulator.new();

const sharedMemoryBuffer = new Uint8Array(
  memory.buffer,
  emu.get_memory(),
  4096
);

const sharedDisplayBuffer = new Uint8Array(
  memory.buffer,
  emu.get_gfx(),
  WIDTH * HEIGHT
);

const sharedKeysBuffer = new Uint8Array(memory.buffer, emu.get_keys(), 16);

const loadGames = () => {
  gamesList.forEach(game => {
    const option = document.createElement('option');
    option.value = game;
    option.innerText = game;
    loadGame.appendChild(option);
  });
};

const initVM = () => {
  canvas.height = HEIGHT * 10;
  canvas.width = WIDTH * 10;
  ctx.fillStyle = 'rgb(0, 0, 0)';
  ctx.fillRect(0, 0, canvas.width, canvas.height);
  emu.load_fontset();
};

const fetchGame = async game =>
  fetch(`roms/${game.toUpperCase()}`)
    .then(res => res.arrayBuffer())
    .then(buffer => {
      const rom = new DataView(buffer, 0, buffer.byteLength);

      for (let i = 0; i < buffer.byteLength; i++) {
        sharedMemoryBuffer[0x200 + i] = rom.getUint8(i);
      }
    });

const handleKeyDown = keyboard => {
  if (keyboard.key in keyboardMapping) {
    sharedKeysBuffer[keyboardMapping[keyboard.key]] = true;
  }
};

const handleKeyUp = keyboard => {
  if (keyboard.key in keyboardMapping) {
    sharedKeysBuffer[keyboardMapping[keyboard.key]] = false;
  }
};

const drawGraphic = () => {
  for (let i = 0; i < WIDTH * HEIGHT; i++) {
    const x = (i % 64) * 10;
    const y = Math.floor(i / 64) * 10;

    ctx.fillStyle = 'rgb(0,0,0)';

    if (sharedDisplayBuffer[i] === 1) {
      ctx.fillStyle = 'rgb(255,255,255)';
    }

    ctx.fillRect(x, y, 10, 10);
  }
};

let running = false;
const runningLoop = () => {
  if (running) {
    for (let i = 0; i < 8; i++) {
      emu.tick();
    }

    if (emu.draw_flag) {
      drawGraphic();
    }
  }

  requestAnimationFrame(runningLoop);
};

loadGame.addEventListener('change', async e => {
  e.target.blur();
  running = false;
  emu.reset();
  await fetchGame(e.target.value);
  initVM();
  running = true;
});

document.addEventListener('keydown', e => handleKeyDown(e));
document.addEventListener('keyup', e => handleKeyUp(e));

loadGames();
initVM();
requestAnimationFrame(runningLoop);
