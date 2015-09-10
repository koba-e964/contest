#include <iostream>
#include <unordered_map>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;

const int N = 5010;
int a[N];

int main(void){
  int l;
  int n;
  cin >> l >> n;
  REP(i, 0, n) {
    cin >> a[i];
  }
  ll cnt = 0;
  unordered_map<int, int> m;
  REP(i, 0, n) {
    REP(j, i + 1, n) {
      int v = a[i] + a[j];
      if (m.count(v) == 0) {
	m[v] = 0;
      }
      m[v] += 2;
    }
  }
  REP(i, 0, n) {
    int v = a[i] * 2;
    if (m.count(v) == 0) {
      m[v] = 0;
    }
    m[v] += 1;
  }
  REP(i, 0, n) {
    if (m.count(l - a[i])) {
      cnt += m[l - a[i]];
    }
  }
  REP(i, 0, n) {
    REP(j, 0, n) {
      if (a[i] * 2 + a[j] == l) {
	cnt -= i != j ? 3 : 1;
      }
    }
  }
  cout << cnt / 6 << endl;
}
