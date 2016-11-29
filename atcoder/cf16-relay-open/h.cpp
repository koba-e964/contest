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



int main(void){
  int n;
  cin >> n;
  int t = 0;
  const int T = 86400;
  const int W = 10801;
  VI freq(T, 0);
  REP(i, 0, n) {
    int a, b;
    cin >> a >> b;
    t += a;
    t %= T;
    freq[t]++;
    t += b;
    t %= T;
  }
  VI cumul(2 * T, 0);
  REP(i, 1, 2 * T) {
    cumul[i] = cumul[i - 1] + freq[(i - 1) % T];
  }
  int ma = 0;
  REP(i, 0, 2 * T - W) {
    ma = max(ma, cumul[i + W] - cumul[i]);
  }
  cout << ma << endl;
}
