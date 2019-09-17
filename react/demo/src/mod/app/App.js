import React, { useState, useEffect } from 'react';
import { List, Icon, Avatar } from 'antd';
import { post } from '../../common/util/request';

import InfiniteScroll from 'react-infinite-scroller';
import './App.css';
// import './mock';

function App({ id = 33337113, type = 0, limit = 10 }) {
  const [list, setList] = useState([]);
  const [offset, setOffset] = useState(0);
  const [before, setBefore] = useState(null);
  const [loading, setLoading] = useState(true);
  const [hasMore, setHasMore] = useState(true);

  useEffect(() => {
    const fetchData = async () => {
      setLoading(true);
      let res = await post('/comment/hot', {
        id: id,
        offset: offset,
        before: before,
        limit: limit,
        type: type
      });
      setHasMore(res.hasMore);
      setList(pre => [...pre, ...res.hotComments]);
      setLoading(false);
    };
    fetchData();
  }, [id, offset, limit, type, before]);

  const loadMore = () => {
    setOffset(list.length);
    setBefore(list[list.length - 1].time);
  };

  return (
    <InfiniteScroll
      pageStart={0}
      initialLoad={false}
      loadMore={loadMore}
      hasMore={!loading && hasMore}
    >
      <List
        itemLayout="vertical"
        size="large"
        dataSource={list}
        loading={loading} // 注释掉显示骨架屏
        renderItem={item => (
          <List.Item
            key={item.commentId}
            actions={[
              <IconText type="star-o" text={item.startCount} />,
              <IconText type="like-o" text={item.likedCount} />
            ]}
            extra={<img width={272} alt="logo" src={item.user.avatarUrl} />}
          >
            {/* <Skeleton loading={loading} active avatar> */}
            <List.Item.Meta
              avatar={<Avatar src={item.user.avatarUrl} />}
              title={item.user.nickname}
              description={item.content}
            />
            {item.content}
            {/* </Skeleton> */}
          </List.Item>
        )}
      />
    </InfiniteScroll>
  );
}

/**
 * 图标与文本
 * @param {*} param0
 */
function IconText({ type, text }) {
  return (
    <span>
      <Icon type={type} style={{ marginRight: 8 }} />
      {text}
    </span>
  );
}

export default App;
