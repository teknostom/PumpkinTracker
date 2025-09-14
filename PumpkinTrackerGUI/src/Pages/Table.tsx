import { useEffect, useState } from "react";
import styles from "./Table.module.css";

interface PrograssData {
	class_name: string;
	methods: { method_name: string; status: string }[];
	percentage_implemented: number;
}

interface ProgressFile {
	component_type: string;
	classes: PrograssData[];
	percentage_implemented: number;
}

interface Props {
	data: ProgressFile;
}

export default ({ data }: Props) => {
	let [selectedMethods, setSelectedMethods] = useState<string[]>([]);
	const allMethods = data
		? data.classes
				.flatMap((cls) => cls.methods)
				.reduce(
					(acc, method) => {
						if (!acc.find((m) => m.method_name === method.method_name)) {
							acc.push(method);
						}
						return acc;
					},
					[] as { method_name: string; status: string }[],
				)
		: [];

	let [filteredData, setFilteredData] = useState<PrograssData[]>(data.classes);

	useEffect(() => {
		if (selectedMethods.length === 0) {
			setFilteredData(data.classes);
			return;
		}
		const filtered = data.classes.filter((cls) =>
			selectedMethods.some((method) =>
				cls.methods.some((m) => m.method_name === method),
			),
		);
		setFilteredData(filtered);
	}, [selectedMethods]);

	return (
		<>
			<div className={styles.header}>
				<h2 className={styles.title}>Pumpkin Implementation Progress</h2>
				<p className={styles.subtitle}>
					Component Type: {data.component_type} | Overall Completion:{" "}
					{data.percentage_implemented.toPrecision(3)}%
				</p>
			</div>
			<div className={styles.filters}>
				<label className={styles.filterLabel}>Filter by Method:</label>
				<div className={styles.filterItem}>
					{allMethods.length > 0 && (
						<div className={styles.methodPills}>
							{allMethods.map((method, index) => (
								<span
									key={index}
									className={`${styles.methodPill} ${
										selectedMethods.includes(method.method_name)
											? styles.selected
											: ""
									}`}
									style={{
										backgroundColor: selectedMethods.includes(
											method.method_name,
										)
											? "#3b82f6"
											: "#e5e7eb",
										color: selectedMethods.includes(method.method_name)
											? "#ffffff"
											: "#000000",
										cursor: "pointer",
									}}
									onClick={() => {
										if (selectedMethods.includes(method.method_name)) {
											setSelectedMethods(
												selectedMethods.filter((m) => m !== method.method_name),
											);
										} else {
											setSelectedMethods([
												...selectedMethods,
												method.method_name,
											]);
										}
									}}
								>
									{method.method_name}
								</span>
							))}
						</div>
					)}
				</div>
			</div>
			<table className={styles.table}>
				<thead>
					<tr>
						<th className={styles.th}>Block</th>
						<th className={styles.th}>Status</th>
						<th className={styles.th}>Methods</th>
					</tr>
				</thead>
				<tbody>
					{data ? (
						filteredData.map((item) => (
							<tr key={item.class_name}>
								<td className={styles.td}>{item.class_name}</td>
								<td className={styles.td}>
									<div className={styles.statusContainer}>
										<div className={styles.progressBarContainer}>
											<div
												className={styles.progressBar}
												title={`${item.percentage_implemented}% implemented`}
												style={{
													width: `${item.percentage_implemented}%`,
													backgroundColor:
														item.percentage_implemented >= 75
															? "#22c55e"
															: item.percentage_implemented >= 50
																? "#f59e0b"
																: "#ef4444",
												}}
											/>
										</div>
									</div>
								</td>
								<td className={styles.td}>
									<div className={styles.methodPills}>
										{item.methods.map((method, index) => (
											<span
												key={index}
												className={styles.methodPill}
												style={{
													backgroundColor:
														method.status === "Implemented"
															? "#4CAF50"
															: "#F44336",
												}}
											>
												{method.method_name}
											</span>
										))}
									</div>
								</td>
							</tr>
						))
					) : (
						<tr>
							<td className={styles.td} colSpan={3}>
								No data available.
							</td>
						</tr>
					)}
				</tbody>
			</table>
		</>
	);
};
