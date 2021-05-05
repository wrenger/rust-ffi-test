
/// A simple 2d vector
struct Point2 {
  float x;
  float y;
};

/// Compute the length
float Point2_len(struct Point2 self);
/// Rotate the vector clockwise around (0,0)
struct Point2 Point2_rotate(struct Point2 self, float radians);
/// Add the two points
struct Point2 Point2_add(struct Point2 self, struct Point2 b);
/// Multiply a scalar to the point
struct Point2 Point2_mul(struct Point2 self, float s);
