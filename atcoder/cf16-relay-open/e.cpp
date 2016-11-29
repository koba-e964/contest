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

ll gcd(ll a, ll b) {
  return b == 0 ? a : gcd(b, a % b);
}

int main(void){
  ll a, b, c, d;
  cin >> a >> b >> c >> d;
  a = abs(c - a);
  b = abs(d - b);
  ll g = gcd(a, b);
  a /= g;
  b /= g;
  cout << g * (a + b - 1) << endl;
}
