import React from 'react'
import SearchBars from './components/SearchBars'
import './styles.css'

export default function App(){
  return (
    <div className="wrap">
      <header><h1>Aegis Browser</h1></header>
      <SearchBars />
      <iframe title="webview" src="about:blank" className="web"></iframe>
    </div>
  )
}
