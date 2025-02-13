<script lang="ts">
	import { createEventDispatcher, onDestroy } from "svelte";

	import { Color, type Gradient } from "@graphite/messages";

	import LayoutCol from "@graphite/components/layout/LayoutCol.svelte";
	import LayoutRow from "@graphite/components/layout/LayoutRow.svelte";

	const dispatch = createEventDispatcher<{ activeMarkerIndexChange: number | undefined; gradient: Gradient }>();

	export let gradient: Gradient;
	export let activeMarkerIndex = 0 as number | undefined;
	// export let disabled = false;
	// export let tooltip: string | undefined = undefined;

	let markerTrack: LayoutRow | undefined;

	function markerPointerDown(e: PointerEvent, index: number) {
		// Left-click to select and begin potentially dragging
		if (e.button === 0) {
			activeMarkerIndex = index;
			dispatch("activeMarkerIndexChange", index);
			addEvents();
			return;
		}

		// Right-click to delete
		if (e.button === 2) {
			deleteStopByIndex(index);
			return;
		}
	}

	function markerPosition(e: MouseEvent): number | undefined {
		const markerTrackRect = markerTrack?.div()?.getBoundingClientRect();
		if (!markerTrackRect) return;

		const ratio = (e.clientX - markerTrackRect.left) / markerTrackRect.width;

		return Math.max(0, Math.min(1, ratio));
	}

	function insertStop(e: MouseEvent) {
		let position = markerPosition(e);
		if (position === undefined) return;

		let before = gradient.stops.findLast((value) => value.position < position);
		let after = gradient.stops.find((value) => value.position > position);

		let color = Color.fromCSS("black") as Color;
		if (before && after) {
			let t = (position - before.position) / (after.position - before.position);
			color = before.color.lerp(after.color, t);
		} else if (before) {
			color = before.color;
		} else if (after) {
			color = after.color;
		}

		let index = gradient.stops.findIndex((value) => value.position > position);
		if (index === -1) index = gradient.stops.length;

		gradient.stops.splice(index, 0, { position, color });
		activeMarkerIndex = index;

		dispatch("activeMarkerIndexChange", index);
		dispatch("gradient", gradient);
	}

	function deleteStop(e: KeyboardEvent) {
		if (e.key.toLowerCase() !== "delete" && e.key.toLowerCase() !== "backspace") return;
		if (activeMarkerIndex === undefined) return;
		deleteStopByIndex(activeMarkerIndex);
	}

	function deleteStopByIndex(index: number) {
		if (gradient.stops.length <= 2) return;

		gradient.stops.splice(index, 1);
		if (gradient.stops.length === 0) {
			activeMarkerIndex = undefined;
		} else {
			activeMarkerIndex = Math.max(0, Math.min(gradient.stops.length - 1, index));
		}

		dispatch("activeMarkerIndexChange", activeMarkerIndex);
		dispatch("gradient", gradient);
	}

	function moveMarker(e: PointerEvent, index: number) {
		// Just in case the mouseup event is lost
		if (e.buttons === 0) removeEvents();

		let position = markerPosition(e);
		if (position === undefined) return;
		setPosition(index, position);
	}

	export function setPosition(index: number, position: number) {
		const active = gradient.stops[index];
		active.position = position;
		gradient.stops.sort((a, b) => a.position - b.position);
		if (gradient.stops.indexOf(active) !== activeMarkerIndex) {
			activeMarkerIndex = gradient.stops.indexOf(active);
			dispatch("activeMarkerIndexChange", gradient.stops.indexOf(active));
		}
		dispatch("gradient", gradient);
	}

	function onPointerMove(e: PointerEvent) {
		if (activeMarkerIndex !== undefined) {
			moveMarker(e, activeMarkerIndex);
		}
	}

	function onPointerUp() {
		removeEvents();
	}

	function addEvents() {
		document.addEventListener("pointermove", onPointerMove);
		document.addEventListener("pointerup", onPointerUp);
	}

	function removeEvents() {
		document.removeEventListener("pointermove", onPointerMove);
		document.removeEventListener("pointerup", onPointerUp);
	}

	document.addEventListener("keydown", deleteStop);
	onDestroy(() => {
		removeEvents();
		document.removeEventListener("keydown", deleteStop);
	});

	// # Backend -> Frontend
	// Populate(gradient, { position, color }[], active) // The only way indexes get changed. Frontend drops marker if it's being dragged.
	// UpdateGradient(gradient)
	// UpdateMarkers({ index, position, color }[])

	// # Frontend -> Backend
	// SendNewActive(index)
	// SendPositions({ index, position }[])
	// AddMarker(position)
	// RemoveMarkers(index[])
	// ResetMarkerToDefault(index)

	// // We need a way to encode constraints on some markers, like locking them in place or preventing reordering
	// // We need a way to encode the allowability of adding new markers between certain markers, or preventing the deletion of certain markers
	// // We need the ability to multi-select markers and move them all at once
</script>

<LayoutCol
	class="spectrum-input"
	styles={{
		"--gradient-start": gradient.firstColor()?.toHexOptionalAlpha() || "black",
		"--gradient-end": gradient.lastColor()?.toHexOptionalAlpha() || "black",
		"--gradient-stops": gradient.toLinearGradientCSS(),
	}}
>
	<LayoutRow class="gradient-strip" on:click={insertStop}></LayoutRow>
	<LayoutRow class="marker-track" bind:this={markerTrack}>
		{#each gradient.stops as marker, index}
			<svg
				style:--marker-position={marker.position}
				style:--marker-color={marker.color.toRgbCSS()}
				class="marker"
				class:active={index === activeMarkerIndex}
				data-gradient-marker
				xmlns="http://www.w3.org/2000/svg"
				viewBox="0 0 12 12"
			>
				<path
					on:pointerdown={(e) => markerPointerDown(e, index)}
					d="M10,11.5H2c-0.8,0-1.5-0.7-1.5-1.5V6.8c0-0.4,0.2-0.8,0.4-1.1L6,0.7l5.1,5.1c0.3,0.3,0.4,0.7,0.4,1.1V10C11.5,10.8,10.8,11.5,10,11.5z"
				/>
				<path
					on:pointerdown={(e) => markerPointerDown(e, index)}
					d="M6,1.4L1.3,6.1C1.1,6.3,1,6.6,1,6.8V10c0,0.6,0.4,1,1,1h8c0.6,0,1-0.4,1-1V6.8c0-0.3-0.1-0.5-0.3-0.7L6,1.4M6,0l5.4,5.4C11.8,5.8,12,6.3,12,6.8V10c0,1.1-0.9,2-2,2H2c-1.1,0-2-0.9-2-2V6.8c0-0.5,0.2-1,0.6-1.4L6,0z"
				/>
			</svg>
		{/each}
	</LayoutRow>
</LayoutCol>

<style lang="scss" global>
	.spectrum-input {
		--marker-half-width: 6px;

		.gradient-strip {
			flex: 0 0 auto;
			height: 16px;
			background-image:
				var(--gradient-stops),
				// Solid start/end colors on either side so the gradient begins at the center of a marker
				linear-gradient(var(--gradient-start), var(--gradient-start)),
				linear-gradient(var(--gradient-end), var(--gradient-end)),
				var(--color-transparent-checkered-background);
			background-size:
				calc(100% - 2 * var(--marker-half-width)) 100%,
				// TODO: Find a solution that avoids visual artifacts where these end colors meet the gradient that appear when viewing with a non-integer zoom or display scaling factor
				var(--marker-half-width) 100%,
				var(--marker-half-width) 100%,
				var(--color-transparent-checkered-background-size);
			background-position:
				var(--marker-half-width) 0,
				left 0,
				right 0,
				var(--color-transparent-checkered-background-position);
			background-repeat: no-repeat, no-repeat, no-repeat, var(--color-transparent-checkered-background-repeat);
			border-radius: 2px;
		}

		.marker-track {
			margin-top: calc(24px - 16px - 12px);
			margin-left: var(--marker-half-width);
			width: calc(100% - 2 * var(--marker-half-width));
			position: relative;

			.marker {
				position: absolute;
				transform: translateX(-50%);
				left: calc(var(--marker-position) * 100%);
				width: 12px;
				height: 12px;

				// Inner fill
				path:first-child {
					fill: var(--marker-color);
				}

				// Outer border
				path:last-child {
					fill: var(--color-5-dullgray);
				}

				&:not(.active) path:first-child:hover + path:last-child,
				&:not(.active) path:last-child:hover {
					fill: var(--color-6-lowergray);
				}

				// Outer border when active
				&.active path:last-child {
					fill: var(--color-e-nearwhite);
				}

				&.active path:first-child:hover + path:last-child,
				&.active path:last-child:hover {
					fill: var(--color-f-white);
				}
			}
		}
	}
</style>
