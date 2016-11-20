#include <algorithm>
#include <bitset>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stack>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;



int main(void){
  int n;
  ll k;
  cin >> n >> k;
  assert (n <= 2000);
  vector<double> pool3, pool2, pool1;
  double pi = acos(-1);
  REP(i, 1, n) {
    REP(j, i + 1, n) {
      set<int> s;
      s.insert(i);
      s.insert(j - i);
      s.insert(n - j);
      double area = (sin((i) * 2 * pi / n));
      area += (sin((j - i) * 2 * pi / n));
      area += (sin((n - j) * 2 * pi / n));
      area /= 2;
      if (s.size() >= 2) {
	pool3.push_back(abs(area));
      }
      if (s.size() == 1)
	pool1.push_back(abs(area));
    }
  }
  sort(pool1.begin(), pool1.end());
  sort(pool3.begin(), pool3.end());
  double lo = 0, hi = 100;
  REP(i, 0, 100) {
    double mid = (lo + hi) / 2;
    int l1 = upper_bound(pool1.begin(), pool1.end(), mid) - pool1.begin();
    int l3 = upper_bound(pool3.begin(), pool3.end(), mid) - pool3.begin();
    if (l1 * n / 3 + l3 * n / 3 >= k) {
      hi = mid;
    } else {
      lo = mid;
    }
  }
  printf("%.15f\n", hi);
}
