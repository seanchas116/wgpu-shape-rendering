import React, { useEffect, useRef } from "react";
import { Renderer } from "../wasm/pkg/wasm";

function App() {
  const containerRef = React.createRef<HTMLDivElement>();
  const rendererRef = useRef<Renderer>();

  useEffect(() => {
    if (containerRef.current) {
      const canvas = document.createElement("canvas");
      canvas.width = 1000;
      canvas.height = 1000;
      canvas.style.width = "500px";
      canvas.style.height = "500px";
      containerRef.current.append(canvas);
      Renderer.new(canvas).then((renderer) => {
        // TODO: cleanup renderer
        rendererRef.current = renderer;
        renderer.render();
      });

      return () => {
        canvas.remove();
      };
    }
  }, []);

  return (
    <div className="p-4">
      <div className="w-fit border border-black" ref={containerRef} />
    </div>
  );
}

export default App;
