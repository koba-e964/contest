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



int main(void) {
  int n;
  cin >> n;
  VL a(n + 1), acc(n + 1);
  a[1] = 1;
  acc[1] = 1;
  REP(i, 2, n + 1) {
    a[i] = i * acc[i - 1] % mod;
    acc[i] = (acc[i - 2] + a[i]) % mod;
  }
  cout << a[n] << endl;
}
