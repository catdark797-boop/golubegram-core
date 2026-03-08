import React, { useState, useEffect } from 'react';
import './App.css';

function App() {
  const [isCatDark, setIsCatDark] = useState(false);
  const [isConnected, setIsConnected] = useState(false);
  const [peerCount, setPeerCount] = useState(0);
  const [qosTokens, setQosTokens] = useState(1500);
  const [logoTaps, setLogoTaps] = useState(0);

  useEffect(() => {
    const timer = setInterval(() => {
      setIsConnected(true);
      setPeerCount(Math.floor(Math.random() * 10) + 1);
    }, 3000);

    return () => clearInterval(timer);
  }, []);

  const handleLogoTap = () => {
    if (isCatDark) return;
    const newTaps = logoTaps + 1;
    setLogoTaps(newTaps);

    if (newTaps >= 5) {
      const pwd = window.prompt("KDF Context Entry:");
      if (pwd === "hunter2") { // Mock valid password
        setIsCatDark(true);
        alert("[CAMOUFLAGE] Matryoshka Layer Shed. Mounting RAM-Drive...");
      }
      setLogoTaps(0);
    }
  };

  const handleNukeNode = () => {
    const isConfirmed = window.confirm(
      "[ВНИМАНИЕ] Активация Scorched Earth Protocol! \nЭтот узел будет немедленно изгнан из Cat_Dark Mesh-сети.\nВсе входящие и исходящие пакеты будут жестко уничтожены."
    );
    if (isConfirmed) {
      alert("🔥 BURN NOTICE ОТПРАВЛЕН 🔥\nMesh-сеть очищается.");
    }
  };

  const handleTopUp = () => {
    alert("Redirection to Monero/LN Gateway for QoS Tokens Top-Up");
    setQosTokens(qosTokens + 500);
  };

  const handlePrivateSwarm = () => {
    alert("Shrodinger Context Initiated. Enter KDF password.");
  }

  const handleSwarmCompute = () => {
    const enable = window.confirm("Enable background LLM/ASI Swarm-Compute?\nThis will burn your battery, but you will earn QoS Tokens.");
    if (enable) {
      alert("Swarm-Compute NPU Worker started.\nMining QoS tokens now.");
    }
  }

  const handleStealthOptics = () => {
    alert("Stealth Optics 120Hz Modulation Ready. Camera background sync is DISABLED per security policy.");
  }

  if (!isCatDark) {
    return (
      <div className="App">
        <header className="App-header">
          <h1 className="innocent-title" onClick={handleLogoTap} style={{ userSelect: 'none', cursor: 'pointer' }}>Голубеграм</h1>
          <h2>Корпоративный Мессенджер</h2>
          <div style={{ margin: '20px', padding: '20px', backgroundColor: '#fff', borderRadius: '8px', boxShadow: '0 2px 5px rgba(0,0,0,0.1)', color: '#333' }}>
            <p>Статус: {isConnected ? "Подключено к серверу" : "Ожидание сети..."}</p>
            <p>Коллег онлайн: {peerCount}</p>
          </div>
          <button className="innocent-btn">Открыть Чаты</button>
        </header>
      </div>
    );
  }

  return (
    <div className="App">
      <header className="App-header cat-dark-mode">
        <h1 className="cat-dark-glitch">CAT_DARK</h1>
        <h2>Enterprise Ghost Mesh</h2>

        <div className="radar-container">
          <p>
            Network Status:
            <span className={isConnected ? "status-active" : ""}>
              {isConnected ? " LINK ESTABLISHED" : " ISOLATED"}
            </span>
          </p>
          <p>Active Peers (BLE/AWDL): {peerCount}</p>
          <p>QoS Priority Credits: {qosTokens}</p>
        </div>

        <button className="nuke-btn" onClick={handleNukeNode}>
          ⚠️ NUKE COMPROMISED NODE
        </button>

        <button className="enterprise-btn" onClick={handleTopUp} style={{ marginTop: '15px' }}>
          [+] TOP-UP QoS CREDITS
        </button>

        <button className="enterprise-btn" onClick={handleSwarmCompute} style={{ marginTop: '15px', background: '#9b59b6', color: '#fff' }}>
          🧠 ENABLE SWARM COMPUTE
        </button>

        <button className="stealth-btn" onClick={handleStealthOptics}>
          STEALTH OPTICAL LINK
        </button>

        <button className="enterprise-btn" onClick={handlePrivateSwarm} style={{ background: '#333', color: '#fff', border: '1px solid #555', marginTop: '15px' }}>
          INITIATE SHRODINGER LAYER
        </button>
      </header>
    </div>
  );
}

export default App;
