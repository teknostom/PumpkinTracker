import type { ReactNode } from "react";
import styles from "./Card.module.css";

export interface Progress {
	fullyImplemented: number;
	mostlyImplemented: number;
	partiallyImplemented: number;
	barelyImplemented: number;
}

interface Props {
	title: string;
	link?: string;
	progress: Progress;
}

function navigateToTable(link?: string) {
	window.location.href = "" + (link ?? "/");
}

export default function Card({ title, link, progress }: Props): ReactNode {
	return (
		<div className={styles.card}>
			<div className={styles.graph}>Graph WIP</div>
			<div className={styles.progressBar}>
				<div
					className={styles.fullyImplemented}
					style={{
						width: progress.fullyImplemented + "%",
						height: "3px",
						backgroundColor: "#238636",
					}}
				></div>
				<div
					className={styles.mostlyImplemented}
					style={{
						width: progress.mostlyImplemented + "%",
						height: "3px",
						backgroundColor: "#d29922",
					}}
				></div>
				<div
					className={styles.partiallyImplemented}
					style={{
						width: progress.partiallyImplemented + "%",
						height: "3px",
						backgroundColor: "#DA7633",
					}}
				></div>
				<div
					className={styles.barelyImplemented}
					style={{
						width: progress.barelyImplemented + "%",
						height: "3px",
						backgroundColor: "#da3633",
					}}
				></div>
				<div
					className={styles.notImplemented}
					style={{
						width:
							"calc(100% - " +
							(progress.fullyImplemented +
								progress.mostlyImplemented +
								progress.partiallyImplemented +
								progress.barelyImplemented) +
							"%)",
						height: "3px",
						backgroundColor: "#e1e4e8",
					}}
				></div>
			</div>
			<div
				className={styles.footer}
				onClick={() => link && navigateToTable(link)}
			>
				<h3>{title}</h3>
				{link && (
					<svg
						width="24"
						height="24"
						viewBox="0 0 24 24"
						fill="none"
						xmlns="http://www.w3.org/2000/svg"
					>
						<path
							d="M8 5L15 12L8 19"
							stroke="#111827"
							strokeWidth="2"
							strokeLinecap="round"
							strokeLinejoin="round"
						/>
					</svg>
				)}
			</div>
		</div>
	);
}
