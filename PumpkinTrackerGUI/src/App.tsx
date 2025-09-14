import styles from "./App.module.css";
import Card, { type Progress } from "./components/Card";

const blockProgress: Progress = {
	fullyImplemented: 10.9,
	mostlyImplemented: 4.0,
	partiallyImplemented: 15.4,
	barelyImplemented: 3.4,
};

const itemProgress: Progress = {
	fullyImplemented: 0.0,
	mostlyImplemented: 0.0,
	partiallyImplemented: 0.0,
	barelyImplemented: 0.0,
};

const entityProgress: Progress = {
	fullyImplemented: 0.0,
	mostlyImplemented: 0.0,
	partiallyImplemented: 0.0,
	barelyImplemented: 0.0,
};

const totalProgress: Progress = {
	fullyImplemented:
		blockProgress.fullyImplemented +
		itemProgress.fullyImplemented +
		entityProgress.fullyImplemented,
	mostlyImplemented:
		blockProgress.mostlyImplemented +
		itemProgress.mostlyImplemented +
		entityProgress.mostlyImplemented,
	partiallyImplemented:
		blockProgress.partiallyImplemented +
		itemProgress.partiallyImplemented +
		entityProgress.partiallyImplemented,
	barelyImplemented:
		blockProgress.barelyImplemented +
		itemProgress.barelyImplemented +
		entityProgress.barelyImplemented,
};

export default () => {
	return (
		<>
			<header>
				<link rel="preconnect" href="https://fonts.googleapis.com" />
				<link rel="preconnect" href="https://fonts.gstatic.com" />
				<link
					href="https://fonts.googleapis.com/css2?family=Montserrat:ital,wght@0,100..900;1,100..900&display=swap"
					rel="stylesheet"
				/>
			</header>
			<div className={styles.content}>
				<Card title="Total" progress={totalProgress} />
				<Card title="Blocks" link="/blocks" progress={blockProgress} />
				<Card title="Items" link="/items" progress={itemProgress} />
				<Card title="Entities" link="/entities" progress={entityProgress} />
			</div>
		</>
	);
};
