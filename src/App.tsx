import { BrowserRouter, Routes, Route, Navigate } from "react-router-dom";
import Layout from "./components/Layout";
import Calculator from "./pages/Calculator";
import Snake from "./pages/Snake";

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Layout />}>
          <Route index element={<Navigate to="/calculator" replace />} />
          <Route path="calculator" element={<Calculator />} />
          <Route path="snake" element={<Snake />} />
        </Route>
      </Routes>
    </BrowserRouter>
  );
}

export default App;
