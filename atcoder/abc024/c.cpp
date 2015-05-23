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

int n,d,k;
const int D=10010, K = 101;
int l[D], r[D];

int main(void){
  cin >> n >> d >> k;
  REP(i, 0, d) {
    cin >> l[i] >> r[i];
  }
  REP(i, 0, k) {
    int s,t;
    cin >> s >> t;
    int c = s;
    int dir = s < t ? 1 : -1;
    REP(j, 0, d) {
      if (c < l[j] || r[j] < c) {
	continue;
      }
      if (l[j] <= t && t <= r[j]) {
	cout << j + 1 << endl;
	break;
      }
      c = (dir == 1)? r[j]: l[j];
    }
  }

}
