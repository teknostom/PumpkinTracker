import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { BrowserRouter, Routes, Route } from "react-router";
import Table from "./Pages/Table";
import blockData from "../../outputs/pumpkin_blocks_analysis.json";

const rootEl = document.getElementById("root");
if (rootEl) {
	const root = ReactDOM.createRoot(rootEl);
	root.render(
		<React.StrictMode>
			<BrowserRouter basename="/PumpkinTracker">
				<Routes>
					<Route path="/blocks" element={<Table data={blockData} />} />
					<Route path="/items" element={<Table data={blockData} />} />
					<Route path="/entities" element={<Table data={blockData} />} />
					<Route path="/" element={<App />} />
				</Routes>
			</BrowserRouter>
		</React.StrictMode>,
	);
}
