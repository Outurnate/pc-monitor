MM = 1;
CM = 10;

LED_HEIGHT = 2.3 * MM;
LED_WIDTH = 7 * MM;
LED_DEPTH = 8 * MM;
LED_HORIZONTAL_CTC = 7.62 * MM;
LED_VERTICAL_CTC = 5.08 * MM;
JIG_MARGIN = 5 * MM;
JIG_THICKNESS = 5 * MM;

LED_COUNT_WIDE = 24;
LED_COUNT_TALL = 10;

difference()
{
    cube([((LED_COUNT_WIDE - 1) * LED_HORIZONTAL_CTC) + (JIG_MARGIN * 2) + LED_WIDTH, ((LED_COUNT_TALL - 1) * LED_VERTICAL_CTC) + (JIG_MARGIN * 2) + LED_HEIGHT, JIG_THICKNESS]);
    translate([JIG_MARGIN, JIG_MARGIN, 0])
    translate([LED_WIDTH / 2, LED_HEIGHT / 2, 0])
    for (y = [0:LED_COUNT_TALL - 1])
    {
        for (x = [0:LED_COUNT_WIDE - 1])
        {
            translate([LED_HORIZONTAL_CTC * x, LED_VERTICAL_CTC * y, 0])
            translate([-LED_WIDTH / 2, -LED_HEIGHT / 2, 0])
            cube([LED_WIDTH, LED_HEIGHT, LED_DEPTH]);
        }
    }
}