#include <algorithm>
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

set<string> pool;

string solve() {
  REP(i, 0, 6) {
    string a = "aaeiuu";
    do {
      string b = "bgmnr";
      do {
	string s;
	REP(j, 0, i) {
	  s += b[j];
	  s += a[j];
	}
	s += a[i];
	REP(j, i, 5) {
	  s += b[j];
	  s += a[j + 1];
	}
	if (!pool.count(s)) {
	  return s;
	}
      } while (next_permutation(b.begin(), b.end()));
    } while (next_permutation(a.begin(), a.end()));
  }
  return "NO";
}
int main(void){
  int n;
  cin >> n;
  REP(i, 0, n) {
    string s;
    cin >> s;
    pool.insert(s);
  }
  cout << solve() << endl;
}
