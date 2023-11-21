// "use client";

import Link from "next/link";
import Image from "next/image";

import { env } from "@/env.mjs";
import { exampleDriver } from "../help/example-driver";

import mapFeature from "public/map-feature-home.png";
import tagLogo from "public/tag-logo.png";
import arrow from "public/icons/arrow-up.svg";
import githubLogo from "public/icons/github.svg";

import softTireIcon from "public/tires/soft.svg";
import mediumTireIcon from "public/tires/medium.svg";
import hardTireIcon from "public/tires/hard.svg";
import intermediateTireIcon from "public/tires/intermediate.svg";
import wetTireIcon from "public/tires/wet.svg";

import DriverMiniSectors from "@/components/DriverMiniSectors";
import DriverTire from "@/components/DriverTire";
import DriverInfo from "@/components/DriverInfo";
import UpNextMeeting from "@/components/UpNext";
import Button from "@/components/Button";

import { NextMeeting } from "@/types/nextMeeting.type";
import HomeRoadmapItem from "../../../components/HomeRoadmapItem";
import HomeFeatureCard from "../../../components/HomeFeatureCard";
import DriverDRS from "../../../components/DriverDRS";

const getNextMeeting = async (): Promise<NextMeeting> => {
	const req = await fetch(`${env.NEXT_PUBLIC_SERVER_URL}/api/next-meeting`, { next: { revalidate: 30 } });
	const res: NextMeeting = await req.json();
	return res;
};

export default async function Page() {
	const nextMeeting = await getNextMeeting();

	return (
		<div>
			<div className="flex min-h-[60vh] w-full flex-col items-center justify-center">
				<Image src={tagLogo} priority alt="f1-dash" className="w-[180px]" />
				<h1 className="text-center text-6xl font-bold text-white">Real-time Formula 1 telemetry and timing</h1>
				<div className="mt-10 flex flex-col items-center gap-10">
					<div className="flex items-center gap-2">
						<Image src={githubLogo} alt="github icon" className="h-8 w-8" />
						<p className="opacity-50">We're open-source</p>
					</div>

					<div>
						<Link href="/">
							<Button>Dashboard</Button>
						</Link>
					</div>
				</div>
			</div>

			<div className="mb-8">
				<p className="text-xl font-semibold text-gray-500">What's our</p>
				<h2 className="mb-4 text-3xl font-bold">Core Features</h2>

				<div className="grid grid-cols-1 gap-2 sm:grid-cols-2 xl:grid-cols-4">
					<HomeFeatureCard
						title="Tires & Pitstops"
						description="Discover which tires the drivers are using and how old they are. As well as their numbers of pitstops."
					>
						<div className="flex space-x-[-1rem] ">
							<Image src={softTireIcon} alt="soft" className="h-8 w-8" />
							<Image src={mediumTireIcon} alt="medium" className="h-8 w-8" />
							<Image src={hardTireIcon} alt="hard" className="h-8 w-8" />
							<Image src={intermediateTireIcon} alt="intermediates" className="h-8 w-8" />
							<Image src={wetTireIcon} alt="wets" className="h-8 w-8" />
						</div>

						<DriverTire stints={exampleDriver.stints} />
					</HomeFeatureCard>

					<HomeFeatureCard
						title="Sector Times"
						description="See Sectore Times and even Minisector data, to figure out whos on a potential personal best or even
						fastest lap."
					>
						<DriverMiniSectors driverDisplayName="test" sectors={exampleDriver.sectors} />
					</HomeFeatureCard>

					<HomeFeatureCard title="Driver Status" description="See at a glace if a driver has DRS or is in the pits">
						<div className="w-16">
							<DriverDRS driverStatus={exampleDriver.status} on possible />
						</div>
						<div className="w-16">
							<DriverDRS driverStatus={"PIT"} on={false} possible={false} />
						</div>

						<DriverInfo laps={exampleDriver.laps} status={exampleDriver.status} />
					</HomeFeatureCard>

					<HomeFeatureCard
						title="Track Map"
						description="See the postion of the drivers in realtime. Where and if drivers crashed and their status."
					>
						<Image src={mapFeature} alt="" className="rounded-lg" />
					</HomeFeatureCard>
				</div>
			</div>

			<div className="mb-8 flex flex-col items-center">
				<UpNextMeeting nextMeeting={nextMeeting} />

				<div className="flex cursor-pointer items-center gap-1 opacity-50">
					<Link href="/archive">Checkout the archive for past sessions</Link>
					<Image src={arrow} alt="arrow right" className="h-4 w-4 rotate-90" />
				</div>
			</div>

			<div className="mb-16">
				<p className="text-xl font-semibold text-gray-500">What's our</p>
				<h2 className="mb-4 text-3xl font-bold">Roadmap</h2>

				<div className="flex flex-wrap gap-2">
					<HomeRoadmapItem>Head-to-Head Page where u can compare 2 Drivers in detail</HomeRoadmapItem>

					<HomeRoadmapItem>Tracklimt violation tracker</HomeRoadmapItem>

					<HomeRoadmapItem>Improve top most UI on dashboard page</HomeRoadmapItem>

					<HomeRoadmapItem>Qualifing Mode, shows the ending laps of drivers during a quali session</HomeRoadmapItem>

					<HomeRoadmapItem>PitStop tracking (TBD)</HomeRoadmapItem>

					<HomeRoadmapItem>Replay of old Sessions from Archive Page</HomeRoadmapItem>
				</div>
			</div>
		</div>
	);
}
