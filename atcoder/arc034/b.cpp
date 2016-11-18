#include <vector>
#include <list>
#include <map>
#include <set>
#include <deque>
#include <stack>
#include <bitset>
#include <algorithm>
#include <functional>
#include <numeric>
#include <utility>
#include <sstream>
#include <iostream>
#include <iomanip>
#include <cstdio>
#include <cmath>
#include <cstdlib>
#include <cctype>
#include <string>
#include <cstring>
#include <ctime>
 
using namespace std;
 
typedef long long ll;
typedef vector<int> VI;
typedef pair<int, int> PI;
const double EPS = 1e-9;
#define rep(i,s,n) for(int i=(s); i < int(n); ++i)
 
const int N = 1e6;
ll n;
 
ll f(ll q) {
  ll s = 0;
  while(q > 0) {
    s += q % 10;
    q /= 10;
  }
  return s;
}
 
int main(void) {
  cin >> n;
  int k = 0;
  for(ll i = n - 200; i < n; ++i) {
    if (i + f(i) == n) { ++k; }
  }
  cout << k << endl;
  for(ll i = n - 200; i < n; ++i) {
    if (i + f(i) == n) {
      cout << i << endl;
    }
  }
  
}
