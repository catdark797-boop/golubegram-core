import { useState, useEffect } from 'react';
import './App.css';

function App() {
  const [isGhostMode, setIsGhostMode] = useState(false);
  const [isConnected, setIsConnected] = useState(false);
  const [peerCount, setPeerCount] = useState(0);
  const [qosTokens, setQosTokens] = useState(1500);
  const [logoTaps, setLogoTaps] = useState(0);
  const [showMarketplace, setShowMarketplace] = useState(false);

  useEffect(() => {
    const timer = setInterval(() => {
      setIsConnected(true);
      setPeerCount(Math.floor(Math.random() * 5) + 1);
      setQosTokens((prev) => prev - Math.floor(Math.random() * 3));
    }, 5000);

    return () => clearInterval(timer);
  }, []);

  const handleLogoTap = () => {
    if (isGhostMode) return;
    setLogoTaps((prev) => {
      const newTaps = prev + 1;
      if (newTaps >= 5) {
        // Matryoshka camouflage trigger
        const password = prompt("Enter Vault PIN");
        if (password === "2024") {
          setIsGhostMode(true);
        } else if (password === "9999") {
          alert("[DURESS PIN DETECTED] Zeroizing all cryptographic material...");
        }
      }
      return newTaps;
    });
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

  if (!isGhostMode) {
    if (showMarketplace) {
      return (
        <div className="App">
          <header className="App-header">
            <h2>P2P Маркетплейс (Swarm Drive)</h2>
            <p style={{ fontSize: '14px', color: '#666' }}>Децентрализованная витрина. Хостинг через IPFS-шарды.</p>
            <div style={{ textAlign: 'left', width: '80%', maxWidth: '600px', margin: '20px 0' }}>

              <div style={{ border: '1px solid #ccc', padding: '15px', marginBottom: '15px', borderRadius: '8px', backgroundColor: '#f9f9f9', color: '#333' }}>
                <h4 style={{ margin: '0 0 10px 0' }}>AnonPhone V3</h4>
                <p style={{ margin: '0 0 10px 0', fontSize: '13px', color: '#888' }}>Источник: SWARM_URI://list_anonphone_v1</p>
                <button className="innocent-btn" style={{ padding: '8px 15px', fontSize: '14px' }} onClick={() => alert("[ESCROW] 0% Fee Smart-Contract Initiated.\nSwarmCompute LLM is currently arbitrating the deal...")}>Купить (Escrow)</button>
              </div>

              <div style={{ border: '1px solid #ccc', padding: '15px', marginBottom: '15px', borderRadius: '8px', backgroundColor: '#f9f9f9', color: '#333' }}>
                <h4 style={{ margin: '0 0 10px 0' }}>Mesh Antenna Kit</h4>
                <p style={{ margin: '0 0 10px 0', fontSize: '13px', color: '#888' }}>Источник: SWARM_URI://list_antenna_v2</p>
                <button className="innocent-btn" style={{ padding: '8px 15px', fontSize: '14px' }} onClick={() => alert("[ESCROW] 0% Fee Smart-Contract Initiated.\nSwarmCompute LLM is currently arbitrating the deal...")}>Купить (Escrow)</button>
              </div>

              <div style={{ border: '1px solid #ccc', padding: '15px', marginBottom: '15px', borderRadius: '8px', backgroundColor: '#f9f9f9', color: '#333' }}>
                <h4 style={{ margin: '0 0 10px 0' }}>Encrypted Router</h4>
                <p style={{ margin: '0 0 10px 0', fontSize: '13px', color: '#888' }}>Источник: SWARM_URI://list_router_v1</p>
                <button className="innocent-btn" style={{ padding: '8px 15px', fontSize: '14px' }} onClick={() => alert("[ESCROW] 0% Fee Smart-Contract Initiated.\nSwarmCompute LLM is currently arbitrating the deal...")}>Купить (Escrow)</button>
              </div>

            </div>
            <button className="innocent-btn" onClick={() => setShowMarketplace(false)}>Назад в Мессенджер</button>
          </header>
        </div>
      );
    }

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
          <button className="innocent-btn" style={{ marginTop: '15px', backgroundColor: '#4b7bec' }} onClick={() => setShowMarketplace(true)}>P2P Маркетплейс</button>
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
