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
const ll mod = 1e9 + 7;

const int N = 100010;
int l[N], r[N];

int main(void){
  int n, m;
  cin >> n >> m;
  int mi = 1e6;
  REP(i, 0, m) {
    cin >> l[i] >> r[i];
    l[i]--, r[i]--;
    mi = min(mi, r[i] - l[i] + 1);
  }
  // mi is optimal.
  VI a(n);
  REP(i, 0, n) {
    a[i] = i % mi;
  }
  printf("%d\n", mi);
  REP(i, 0, n) {
    printf("%d%c", a[i], i == n - 1 ? '\n' : ' ');
  }
}
