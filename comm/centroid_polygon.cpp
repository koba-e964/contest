/*
 * Typical T: complex<double>, complex<long double>
 * Header requirement: vector
 * Verified by: http://codeforces.com/contest/975/submission/37828527
 * Reference: https://en.wikipedia.org/wiki/Centroid#Centroid_of_a_polygon
 */
template<class T>
T centroid_polygon(const std::vector<T> &pts) {
  typedef typename T::value_type ty;
  int n = pts.size();
  ty area2 = 0; // 2 * area
  T cent = 0;
  T bias = pts[0]; // To avoid loss of accuracy
  for (int i = 0; i < n; ++i) {
    ty outer = (conj(pts[i] - bias) * (pts[(i + 1) % n] - bias)).imag();
    area2 += outer;
    cent += ((pts[i] - bias) + (pts[(i + 1) % n] - bias)) * outer;
  }
  return cent / T(3 * area2, 0) + bias;
}
