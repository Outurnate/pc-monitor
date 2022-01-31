model: (model-stl "led_jig")

model-stl NAME:
	openscad -o dist/{{NAME}}.stl model/{{NAME}}.scad
