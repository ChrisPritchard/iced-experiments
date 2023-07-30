# SVG Map

A world map with points on it highlighted.

The way this will likely work is by having a custom widget:

- svg, for the base layer
- map_point vector, which are all drawn in the overlay layer

each map_point is a custom widget that takes an x,y, maybe has some animation, and exposes on click etc

the overlay for the custom widget renders a column of these map_points, so can be modelled after that sort of collection mechanic.