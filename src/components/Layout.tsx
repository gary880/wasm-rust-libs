import { Link, Outlet, useLocation } from "react-router-dom";

const Layout = () => {
  const location = useLocation();

  const getLinkStyle = (path: string) => ({
    padding: "8px 16px",
    borderRadius: "4px",
    textDecoration: "none",
    color: location.pathname === path ? "#646cff" : "#fff",
    backgroundColor: location.pathname === path ? "#1a1a1a" : "transparent",
    fontWeight: 600,
    transition: "all 0.2s",
  });

  return (
    <div
      style={{ minHeight: "100vh", display: "flex", flexDirection: "column" }}
    >
      <nav
        style={{
          display: "flex",
          gap: "20px",
          padding: "20px 40px",
          backgroundColor: "#1a1a1a",
          borderBottom: "1px solid #333",
        }}
      >
        <Link to="/calculator" style={getLinkStyle("/calculator")}>
          Calculator
        </Link>
        <Link to="/snake" style={getLinkStyle("/snake")}>
          Snake Game
        </Link>
      </nav>

      <main
        style={{
          flex: 1,
          padding: "40px",
          display: "flex",
          justifyContent: "center",
        }}
      >
        <div style={{ width: "100%", maxWidth: "800px" }}>
          <Outlet />
        </div>
      </main>
    </div>
  );
};

export default Layout;
