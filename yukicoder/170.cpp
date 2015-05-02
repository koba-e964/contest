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
typedef pair<int, int> PI;
const double EPS=1e-9;


int f[26];

int main(void){
  string s;
  cin >> s;
  int l = s.length();
  REP(i, 0, l) {
    f[s[i] - 'A']++;
  }
  int q = 1;
  REP(i, 1, l + 1) {
    q *= i;
  }
  REP(i, 0, 26) {
    REP(j, 1, f[i] + 1) {
      q /= j;
    }
  }
  cout << q - 1 << endl;
}
