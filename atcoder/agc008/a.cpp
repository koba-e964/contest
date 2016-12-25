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

const ll inf = 1e16;

ll solve(ll x, ll y) {
  return x <= y ? y - x : inf;
}

int main(void){
  ll x, y;
  cin >> x >> y;
  ll mi = solve(x, y);
  mi = min(mi, solve(x, -y) + 1);
  mi = min(mi, solve(-x, y) + 1);
  mi = min(mi, solve(-x, -y) + 2);
  cout << mi << endl;
}
