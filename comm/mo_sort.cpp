/**
 * Use Mo's algorithm to rearrange a sequence of points.
 * 'queries' consists of pairs ((l, r), idx).
 * Header Requirement: vector, utility
 */
struct mo_cmp {
  int b;
  mo_cmp(int b) : b(b) {}
  bool operator()(std::pair<std::pair<int, int>, int> x,
		  std::pair<std::pair<int, int>, int> y) {
    std::pair<int, int> xp = x.first, yp = y.first;
    if (xp.first / b != yp.first / b) {
      return xp.first / b < yp.first / b;
    }
    int dir = (xp.first / b) % 2;
    return (xp.second < yp.second) ^ dir;
  }
};
void mo_sort(std::vector<pair<pair<int, int>, int> > &queries,
	     int b) {
  sort(queries.begin(), queries.end(), mo_cmp(b));
}
