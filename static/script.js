document.addEventListener("DOMContentLoaded", function () {
    var map = L.map("map").setView([48.8588443, 2.2943506], 13);
    L.tileLayer("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png", {
        attribution: "&copy; OpenStreetMap contributors",
    }).addTo(map);

    new L.GPX("/static/map.gpx", { async: true })
        .on("loaded", function (e) { map.fitBounds(e.target.getBounds()); })
        .addTo(map);
});
