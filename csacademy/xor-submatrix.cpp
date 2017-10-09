#include <iostream>
#include <cassert>
#include <vector>

using namespace std;
typedef vector<int> VI;
#define rep(i, n) for (int i = 0; i < int(n); ++i)

struct trie {
    trie *child[2];
    trie() {
        child[0] = nullptr;
        child[1] = nullptr;
    }
} root;

const int TRIE_SIZE = 15000000;
trie res[TRIE_SIZE];
int res_ptr = 0;


const int B = 30;
void add(int x) {
    trie *cur = &root;
    for (int i = B - 1; i >= 0; --i) {
        int v = (x >> i) & 1;
        if (cur->child[v] == nullptr) {
            cur->child[v] = &res[res_ptr++];
        }
        cur = cur->child[v];
    }
}

int query_max(int x) {
    trie *cur = &root;
    int tmp = 0;
    for (int i = B - 1; i >= 0; --i) {
        int v = (x >> i) & 1;
        if (cur->child[1 - v] != nullptr) {
            tmp |= 1 << i;
            cur = cur->child[1 - v];
        } else {
            assert (cur->child[v] != nullptr);
            cur = cur->child[v];
        }
    }
    return tmp;
}


int calc(VI v_odd, VI u_odd) {
    int n = v_odd.size();
    int m = u_odd.size();
    int ma = 0;
    rep(i, n) {
        add(v_odd[i]);
    }
    rep(j, m) {
        ma = max(ma, query_max(u_odd[j]));
    }
    return ma;
}

int main() {
    int n, m;
    cin >> n >> m;
    vector<int> v(n), u(m);
    vector<int> v_acc(n + 1, 0), u_acc(m + 1);
    rep(i, n) {
        cin >> v[i];
        v_acc[i + 1] = v_acc[i] ^ v[i];
    }
    rep(j, m) {
        cin >> u[j];
        u_acc[j + 1] = u_acc[j] ^ u[j];
    }
    int ma = 0;
    // one side even, the other odd
    rep(i, n + 1) {
        rep(j, i) {
            if ((i - j) % 2 == 0) {
                ma = max(ma, v_acc[j] ^ v_acc[i]);
            }
        }
    }
    rep(i, m + 1) {
        rep(j, i) {
            if ((i - j) % 2 == 0) {
                ma = max(ma, u_acc[j] ^ u_acc[i]);
            }
        }
    }
    VI v_odd, u_odd;
    rep(i, n + 1) {
        rep(j, i) {
            if ((i - j) % 2 == 1) {
                v_odd.push_back(v_acc[j] ^ v_acc[i]);
            }
        }
    }
    rep(i, m + 1) {
        rep(j, i) {
            if ((i - j) % 2 == 1) {
                u_odd.push_back(u_acc[j] ^ u_acc[i]);
            }
        }
    }
    ma = max(ma, calc(v_odd, u_odd));
    cout << ma << endl;
}
