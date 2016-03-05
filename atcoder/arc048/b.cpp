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

const int N = 100100;
int r[N], h[N];
int tbl[N][3];
int acc[N];
int main(void){
  int n;
  cin >> n;
  REP(i, 0, n) {
    cin >> r[i] >> h[i];
    h[i]--;
    tbl[r[i]][h[i]]++;
  }
  REP(i, 1, N) {
    acc[i] = acc[i - 1] + tbl[i][0] + tbl[i][1] + tbl[i][2]; 
  }
  REP(i, 0, n) {
    int win = acc[r[i] - 1];
    win += tbl[r[i]][(h[i] + 1) % 3];
    int draw = tbl[r[i]][h[i]] - 1;
    int lose = n - 1 - win - draw;
    cout << win << " " << lose << " "  << draw << endl;
  }
}
