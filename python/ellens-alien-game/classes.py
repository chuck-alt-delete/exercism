"""Solution to Ellen's Alien Game exercise."""


class Alien:
    """Create an Alien object with location x_coordinate and y_coordinate.

    Attributes
    ----------
    (class)total_aliens_created: int
    x_coordinate: int - Position on the x-axis.
    y_coordinate: int - Position on the y-axis.
    health: int - Amount of health points.

    Methods
    -------
    hit(): Decrement Alien health by one point.
    is_alive(): Return a boolean for if Alien is alive (if health is > 0).
    teleport(new_x_coordinate, new_y_coordinate): Move Alien object to new coordinates.
    collision_detection(other): Implementation TBD.
    """

    total_aliens_created = 0

    @classmethod
    def update_total_aliens_created(cls):
        cls.total_aliens_created += 1


    def __init__(self,x_coordinate: int, y_coordinate: int) -> None:
        self.x_coordinate = x_coordinate
        self.y_coordinate = y_coordinate
        self.health = 3
        Alien.update_total_aliens_created()

    def hit(self):
        self.health -= 1
    
    def is_alive(self):
        return self.health > 0

    def teleport(self, new_x_coordinate, new_y_coordinate):
        self.x_coordinate = new_x_coordinate
        self.y_coordinate = new_y_coordinate

    def collision_detection(self,other):
        pass


def new_aliens_collection(coordinate_list :list[tuple]) -> list[Alien]:
    """Create a list of aliens from a list of coordinates"""
    aliens = []
    for x_coordinate, y_coordinate in coordinate_list:
        aliens.append(Alien(x_coordinate=x_coordinate,y_coordinate=y_coordinate))
    return aliens
