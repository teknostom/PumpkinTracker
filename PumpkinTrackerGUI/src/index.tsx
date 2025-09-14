import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { Routes, Route, HashRouter } from "react-router";
import Table from "./Pages/Table";
import blockData from "../../outputs/pumpkin_blocks_analysis.json";

const rootEl = document.getElementById("root");
if (rootEl) {
	const root = ReactDOM.createRoot(rootEl);
	root.render(
		<React.StrictMode>
			<HashRouter>
				<Routes>
					<Route path="/blocks" element={<Table data={blockData} />} />
					<Route path="/items" element={<Table data={blockData} />} />
					<Route path="/entities" element={<Table data={blockData} />} />
					<Route path="/" element={<App />} />
				</Routes>
			</HashRouter>
		</React.StrictMode>,
	);
}
