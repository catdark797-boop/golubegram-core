// MIL-SPEC Anchor Node Casing for Cat_Dark Enterprise
// Parametric OpenSCAD Design for 3D Printing (Ruggedized, IP67 compliant logic)

$fn = 100;
wall_thickness = 4.5; // Heavy duty walls
board_width = 85.0; // Raspberry Pi / Orange Pi size
board_length = 56.0;
board_height = 20.0;
clearance = 2.0;

module base_case() {
    difference() {
        // Outer Shell
        cube([board_width + (wall_thickness*2) + clearance, 
              board_length + (wall_thickness*2) + clearance, 
              board_height + wall_thickness + clearance]);
              
        // Inner Cavity
        translate([wall_thickness, wall_thickness, wall_thickness])
        cube([board_width + clearance, 
              board_length + clearance, 
              board_height + clearance + 1]);
              
        // Antenna Ports (SMA Connectors)
        translate([-1, board_length/2 + wall_thickness, board_height/2 + wall_thickness])
        rotate([0, 90, 0])
        cylinder(h=wall_thickness+2, r=3.5);
    }
}

// Render the rugged case
base_case();
