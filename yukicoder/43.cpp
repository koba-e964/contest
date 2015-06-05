#include <algorithm>
#include <bitset>
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

string s[10];
vector<PI> t;
int main(void){
  int n;
  cin >> n;
  REP(i,0,n) {
    cin >> s[i];
  }
  REP(i, 0, n) {
    REP(j, i + 1, n) {
      if (s[i][j] == '-') {
	t.push_back(PI(i, j));
      }
    }
  }
  int c = t.size();
  int mi = 1000;
  REP(bits, 0, 1 << c) {
    int a[10] = {0};
    REP(i, 0, n) {
      REP(j,0,n) {
	a[i] += s[i][j] == 'o';
      }
    }
    REP(i, 0, c) {
      if (bits & (1 << i)) {
	a[t[i].second]++;
      } else {
	a[t[i].first]++;
      }
    }
    set<int> s;
    REP(i, 0, n) {
      if (a[0] <= a[i]) s.insert(a[i]);
    }
    mi = min(mi, (int)s.size());
  }
  cout << mi << endl;
}
