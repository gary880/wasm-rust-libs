const SnakePage = () => {
  return (
    <div>
      <h2>Snake Game (WASM Engine)</h2>
      <div style={{ width: "400px", height: "400px", background: "#000" }}>
        {/* 未來的 Canvas 會放在這裡 */}
        <p style={{ color: "#fff", textAlign: "center", paddingTop: "180px" }}>
          Canvas Engine Loading...
        </p>
      </div>
      <p>提示：準備開始實作 snake_engine 模組</p>
    </div>
  );
};

export default SnakePage;
