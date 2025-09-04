import { useState } from 'react'

export default function SearchBars(){
  const [apps, setApps] = useState('')
  const [leisure, setLeisure] = useState('')
  const [sites, setSites] = useState('')

  const go = (type, q) => {
    const base = 'http://localhost:8088/search'
    window.location.href = `${base}/${type}?q=${encodeURIComponent(q)}`
  }

  return (
    <div className="grid">
      <div>
        <input placeholder="Apps Web/Android/Linux/Mac" value={apps} onChange={e=>setApps(e.target.value)} />
        <button onClick={()=>go('apps', apps)}>Chercher</button>
      </div>
      <div>
        <input placeholder="Loisirs / Achats / Jeux / Design" value={leisure} onChange={e=>setLeisure(e.target.value)} />
        <button onClick={()=>go('leisure', leisure)}>Chercher</button>
      </div>
      <div>
        <input placeholder="Sites à usage massif" value={sites} onChange={e=>setSites(e.target.value)} />
        <button onClick={()=>go('sites', sites)}>Chercher</button>
      </div>
    </div>
  )
}
