#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;

const int N = 123456;
vector<int> pool[N];

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, k, m;
  cin >> n >> k >> m;
  REP(i, 0, n) {
    int a;
    cin >> a;
    pool[a % m].push_back(a);
  }
  REP(i, 0, m) {
    if ((int) pool[i].size() >= k) {
      cout << "Yes\n";
      REP(j, 0, k) {
	cout << pool[i][j] << (j == k - 1 ? "\n" : " ");
      }
      return 0;
    }
  }
  cout << "No\n";
}
