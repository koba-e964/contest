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

const int N = 243700;
int a[N];

int main(void){
  int n, k, q;
  cin >> n >> k >> q;
  REP(i, 0, n) {
    int l, r;
    cin >> l >> r;
    a[l] += 1;
    a[r + 1] -= 1;
  }
  REP(i, 0, N - 1) {
    a[i + 1] += a[i];
  }
  REP(i, 0, N - 1) {
    a[i + 1] = (a[i + 1] >= k ? 1 : 0) + a[i];
  }
  REP(i, 0, q) {
    int u, v;
    cin >> u >> v;
    cout << a[v] - a[u - 1] << endl;
  }
}
