import { useState, useEffect } from "react";
import init, { Calculator } from "../../rust-libs/calculator/pkg/calculator";

const CalculatorPage = () => {
  const [input, setInput] = useState("2 + 2");
  const [result, setResult] = useState<string>("");
  const [ready, setReady] = useState(false);

  useEffect(() => {
    init().then(() => setReady(true));
  }, []);

  const handleCalc = () => {
    if (!ready) return;
    try {
      setResult(Calculator.eval(input).toString());
    } catch (err: unknown) {
      if (err instanceof Error) {
        // 這裡 err 自動收窄為 Error 型別
        setResult(`解析錯誤: ${err.message}`);
      } else {
        setResult("發生未知錯誤");
      }
    }
  };

  return (
    <div
      style={{
        backgroundColor: "#2a2a2a",
        padding: "30px",
        borderRadius: "12px",
        boxShadow: "0 8px 24px rgba(0,0,0,0.3)",
      }}
    >
      <h2 style={{ marginTop: 0, color: "#646cff" }}>
        WASM Scientific Calculator
      </h2>

      <div style={{ display: "flex", gap: "10px", marginBottom: "20px" }}>
        <input
          value={input}
          onChange={(e) => setInput(e.target.value)}
          placeholder="e.g. 2 + 2 * sin(pi/2)"
          style={{
            flex: 1,
            padding: "12px",
            borderRadius: "6px",
            border: "1px solid #444",
            backgroundColor: "#1a1a1a",
            color: "#fff",
            fontSize: "16px",
          }}
        />
        <button
          onClick={handleCalc}
          style={{
            padding: "0 24px",
            backgroundColor: "#646cff",
            border: "none",
            borderRadius: "6px",
            color: "#fff",
            cursor: "pointer",
            fontWeight: "bold",
          }}
        >
          Run
        </button>
      </div>

      <div
        style={{
          padding: "20px",
          backgroundColor: "#1a1a1a",
          borderRadius: "6px",
          minHeight: "60px",
          border: "1px dashed #444",
        }}
      >
        <span style={{ color: "#888", fontSize: "14px" }}>Result:</span>
        <div
          style={{
            fontSize: "24px",
            fontWeight: "bold",
            color: "#4ade80",
            marginTop: "4px",
          }}
        >
          {result || "---"}
        </div>
      </div>
    </div>
  );
};

export default CalculatorPage;
