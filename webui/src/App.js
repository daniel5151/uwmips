import React, { useState } from 'react';
import './App.css';

import AceEditor from 'react-ace';

import './ace_syntax/uwmips';
import 'brace/theme/monokai';
import 'brace/ext/searchbox';

import sample from './asm_samples/recsum';

const Loaded = ({ wasm }) => <button onClick={() => wasm.greet("boi")}>Click me</button>;

const Unloaded = ({ loading, loadWasm }) => {
  return loading ? (
    <div>Loading...</div>
  ) : (
    <button onClick={loadWasm}>Load library</button>
  );
};

function App() {
  const [loading, setLoading] = useState(false);
  const [wasm, setWasm] = useState(null);

  const loadWasm = async () => {
    try {
      setLoading(true);
      const wasm = await import('./uwmips');
      setWasm(wasm);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="App">
      <AceEditor
        // annotations={this.state.annotations}
        width="100%"
        height="100%"
        mode="mips_assembler"
        theme="monokai"
        // onChange={this.handleCodeChange}
        name="codeeditor"
        editorProps={{$blockScrolling: 1}}
        value={sample}
      />

      {wasm ? (
        <Loaded wasm={wasm} />
      ) : (
        <Unloaded loading={loading} loadWasm={loadWasm} />
      )}
    </div>
  );
}

export default App;