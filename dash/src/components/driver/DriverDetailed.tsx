"use client";

import { motion } from "framer-motion";
import clsx from "clsx";

import DriverHistoryTires from "@/components/driver/DriverHistoryTires";
import Graph from "@/components/Graph";

import { TimingAppDataDriver } from "@/types/state.type";

type Props = {
	history: undefined;
	appTimingDriver: TimingAppDataDriver;
};

const maxLength = (values: number[], max: number): number[] => {
	return values.slice(-max);
};

export default function DriverDetailed({ history, appTimingDriver }: Props) {
	return (
		<motion.div
			initial={{ opacity: 0 }}
			exit={{ opacity: 0 }}
			animate={{ opacity: 1 }}
			className={clsx("grid items-center gap-2")}
			style={{
				gridTemplateColumns: "10rem 9.5rem 5rem 5rem auto",
			}}
		>
			<div className="flex flex-col gap-1  place-self-start text-sm font-medium leading-none text-zinc-600">
				<p>Expected Box in 3L</p>
				<p>Average Pit: 22s</p>
				<p>Expected re-join 8th</p>
			</div>

			<div className="w-full">
				<DriverHistoryTires stints={appTimingDriver.stints} />
			</div>

			<div className="">
				<Graph values={maxLength([], 13)} lines={13} />
			</div>
			<div className="">
				<Graph values={maxLength([], 13)} lines={13} />
			</div>

			<div className="flex w-full justify-between gap-2">
				<div>
					<Graph values={maxLength([] ?? [], 13)} lines={13} />
				</div>
				<div>
					<Graph values={maxLength([], 13)} lines={13} />
				</div>
				<div>
					<Graph values={maxLength([], 13)} lines={13} />
				</div>
			</div>
		</motion.div>
	);
}