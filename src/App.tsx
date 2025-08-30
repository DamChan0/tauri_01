import { useEffect, useState } from "react";
import reactLogo from "./assets/react.svg";
import { api } from "./lib/tauri";
import { invoke } from "@tauri-apps/api/core";

export default function App() {
  const [color, setColor] = useState<string>("#22c55e");

  useEffect(() => {
    api.getColor().then(setColor);
    const off = api.onColorChanged(setColor);
    return () => {
      off.then((un) => un());
    };
  }, []);

  const apply = (c: string) => {
    setColor(c);
    api.setColor(c).catch(() => api.getColor().then(setColor));
  };

  return (
    <div style={{ padding: 16 }}>
      <h2>색상 수련장</h2>

      <div
        style={{
          width: 200,
          height: 120,
          border: "1px solid #ccc",
          background: color,
          marginBottom: 12,
        }}
      />

      <input type="color" value={color} onChange={(e) => apply(e.target.value)} />

      <button style={{ marginLeft: 8 }} onClick={() => apply("#3b82f6")}>
        푸른 기 주입
      </button>
      <button style={{ marginLeft: 8 }} onClick={() => apply("#ef4444")}>
        붉은 기 주입
      </button>
    </div>
  );
}
