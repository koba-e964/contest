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


int n, k;
int t[5][5];

int rec(int v, int sum) {
  if (v >= n) {
    return sum == 0;
  }
  int acc = 0;
  REP(i, 0, k) {
    acc |= rec(v + 1, sum ^ t[v][i]);
  }
  return acc;
}

int main(void){
  cin >> n >> k;
  REP(i, 0, n) {
    REP(j, 0, k) {
      cin >> t[i][j];
    }
  }
  cout << (rec(0, 0) ? "Found" : "Nothing") << endl;
}
