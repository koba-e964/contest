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

int a[26] = {};

int main(void) {
  REP(i, 0, 26) {
    cin >> a[i];
  }
  ll sum = 1;
  sum *= a['d' - 'a'];
  sum *= a['e' - 'a'];
  sum *= a['h' - 'a'];
  sum *= a['w' - 'a'];
  sum *= a['r' - 'a'];
  int o = a['o' - 'a'];
  sum *= o / 2 * (o - o / 2);
  int l = a['l' - 'a'];
  int ma = 0;
  REP(i, 0, l) {
    ma = max(ma, i * (i - 1) / 2 * (l - i));
  }
  sum *= ma;
  cout << sum << endl;
}
