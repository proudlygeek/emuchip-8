/* tslint:disable */
export class Emulator {
free(): void;

static  new(): Emulator;

 load_fontset(): void;

 get_memory(): number;

 get_gfx(): number;

 get_keys(): number;

 tick(): void;

 draw_flag(): boolean;

 reset(): void;

}
