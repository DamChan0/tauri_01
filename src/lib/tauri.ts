import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

// 객체 리터럴
export const api = {
  // 색상
  getColor: () => invoke<string>("get_color"),
  setColor: (color: string) => invoke<string>("set_color", { color }),

  /* call :(color: string) => void  = void를 반환하는
   (color: string) => void 라는 함수가 있는데 이것이 매개변수다.
   즉, call은 콜백함수이이고 onColorChanged를 콜백함수가 호출 될떄 동작하는 
   구현 부분이다.
  */
  onColorChanged: (call: (color: string) => void): Promise<UnlistenFn> =>
    listen<string>("color_changed", (e) => call(e.payload)),

  // 카운터
  getCount: () => invoke<number>("get_count"),
  increment: (amount?: number) => invoke<number>("increment", { amount }),
  resetCount: () => invoke<number>("reset_count"),

  // 드롭다운
  getLevel: () => invoke<string>("get_level"),
  setLevel: (level: string) => invoke<string>("set_level", { level }),

  // 스크롤(페이징)
  getItems: (offset: number, limit: number) =>
    invoke<{ items: string[]; next_offset: number }>("get_items", { offset, limit }),
};
