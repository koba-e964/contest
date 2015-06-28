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

const int N = 1e5 + 10;
vector<int> a[N];

int main(void){
  int n, k;
  cin >> n >> k;
  int tot = n - 1;
  REP(i, 0, k) {
    int m;
    cin >> m;
    a[i].reserve(m);
    REP(j, 0, m) {
      int q;
      cin >> q;
      a[i].push_back(q);
    }
    int c = 0;
    for (int j = 1; j < m; j++) {
      if (a[i][j - 1] + 1 == a[i][j]) {
	c++;
      } else {
	break;
      }
    }
    if (a[i][0] != 1) {
      c = 0;
    }
    tot -= c;
    tot += m - c - 1;
  }
  cout << tot << endl;
}
