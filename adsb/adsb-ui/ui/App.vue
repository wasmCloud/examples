<script setup>
import { ref, inject, onMounted } from 'vue'

const center = ref([-98.5, 38])
const projection = ref('EPSG:4326')
const zoom = ref(4.9)
const rotation = ref(0)

const clusterDistance = ref(40)
const radius = ref(10)
const strokeWidth = ref(2)
const strokeColor = ref('blue')

const format = inject('ol-format');
const geoJson = new format.GeoJSON();

const contacts = ref(null)
const stations = ref(null)

function getIconUrl(type) {
    if (type === "s") {
        return new URL("./assets/station.png", import.meta.url).href
    }
    if (type === "c") {
        return new URL("./assets/plane.png", import.meta.url).href
    }
}

onMounted(() => {
    fetch("http://localhost:8085/contacts")
        .then(response => response.json())
        .then(data => (contacts.value = data.contacts));
    fetch("http://localhost:8085/stations")
        .then(response => response.json())
        .then(data => (stations.value = data.stations));

    setInterval(() => {
        fetch("http://localhost:8085/contacts")
            .then(response => response.json())
            .then(data => (contacts.value = data.contacts));
        fetch("http://localhost:8085/stations")
            .then(response => response.json())
            .then(data => (stations.value = data.stations));
    }, 3000);

})

function getCoords(station) {
    return [station.geometry.coordinates[1], station.geometry.coordinates[0]]
}

function getName(station) {
    if (station.properties.type === "contact") {
        return station.properties.icao.toString()
    }
    if (station.properties.type === "station") {
        if (station.properties.name !== "") {
            return station.properties.name
        } else {
            return station.properties.id
        }
    }
}

</script>



<template>
    <div class="container mx-auto min-h-full min-w-full px-4">
        <div class="flex flex-row">
            <ol-map :loadTilesWhileAnimating="true" :loadTilesWhileInteracting="true" style="height:800px">
                <ol-view ref="view" :center="center" :rotation="rotation" :zoom="zoom" :projection="projection" />
                <ol-tile-layer>
                    <ol-source-osm />
                </ol-tile-layer>

                <ol-vector-layer>
                    <ol-source-vector>
                        <ol-feature v-for="station in stations" :key="station.properties.id">
                            <ol-geom-point :coordinates="getCoords(station)"></ol-geom-point>
                            <ol-style>
                                <ol-style-stroke :color="strokeColor" :width="strokeWidth"></ol-style-stroke>
                                <ol-style-text :text="getName(station)">
                                    <ol-style-fill color="red"></ol-style-fill>
                                </ol-style-text>
                                <ol-style-icon :src="getIconUrl('s')" :scale="0.1"></ol-style-icon>
                            </ol-style>
                        </ol-feature>
                        <ol-feature v-for="contact in contacts" :key="contact.properties.icao">
                            <ol-geom-point :coordinates="getCoords(contact)"></ol-geom-point>
                            <ol-style>
                                <ol-style-text color="white" :text="getName(contact)">
                                    <ol-style-fill color="red"></ol-style-fill>
                                </ol-style-text>
                                <ol-style-icon :rotation="180" :src="getIconUrl('c')" :scale="0.1"></ol-style-icon>
                            </ol-style>
                        </ol-feature>
                    </ol-source-vector>
                </ol-vector-layer>
            </ol-map>
        </div>
    </div>
</template>

