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
typedef pair<int, int> PI;
const double EPS=1e-9;



int main(void){
  int n, m;
  cin >> n >> m;
  VI ap, an;
  int zero = 0;
  REP(i, 0, m) {
    int q;
    cin >> q;
    if (q < 0) {
      an.push_back(q);
    } else if (q > 0) {
      ap.push_back(q);
    } else {
      zero++;
    }
  }
  if (zero >= 1) {
    n -= 1;
    m -= 1;
  }
  sort(ap.begin(), ap.end());
  sort(an.rbegin(), an.rend());
  if (n == 0) {
    cout << 0 << endl;
    return 0;
  }
  int mi = 0x3fffffff;
  REP(i, 0, min((int) ap.size(), n - 1)) {
    if (n - i - 1 <= an.size())
      mi = min(mi, 2 * ap[i] - an[n - i - 2]);
  }
  REP(i, 0, min((int) an.size(), n - 1)) {
    if (n - i - 1 <= ap.size())
      mi = min(mi, 2 * (- an[i]) + ap[n - i - 2]);
  }
  if (n <= ap.size()) mi = min(mi, ap[n - 1]);
  if (n <= an.size()) mi = min(mi, -an[n - 1]);
  cout << mi << endl;
}
