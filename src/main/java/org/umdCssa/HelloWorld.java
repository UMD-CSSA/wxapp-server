package org.umdCssa;

import com.firenio.codec.http11.HttpCodec;
import com.firenio.codec.http11.HttpFrame;
import com.firenio.component.Channel;
import com.firenio.component.ChannelAcceptor;
import com.firenio.component.Frame;
import com.firenio.component.IoEventHandle;
import com.firenio.component.LoggerChannelOpenListener;
import com.firenio.component.NioEventLoopGroup;

import org.jetbrains.annotations.NotNull;

import java.time.ZoneId;
import java.time.ZonedDateTime;
import java.time.format.DateTimeFormatter;

public class HelloWorld {

    public static void main(@NotNull final String[] args) throws Exception {

        final var ioEventHandle = new IoEventHandle() {

            @Override
            public void accept(@NotNull final Channel channel,
                               @NotNull final Frame frame) throws Exception {
                final HttpFrame httpFrame = ((HttpFrame) frame);
                final String
                        url = httpFrame.getRequestURL(),
                        text = httpFrame.getArrayContent() != null
                                ? new String(httpFrame.getArrayContent())
                                : "[NOTHING]";
                final String content = "" +
                        "Hello World!\n" +
                        "\n" +
                        "url=" + url + "\n" +
                        "post=" + text + "\n" +
                        "\n" +
                        "\n" +
                        "\n" +
                        ZonedDateTime.now(ZoneId.of("America/New_York"))
                                .format(DateTimeFormatter.ISO_ZONED_DATE_TIME);

                httpFrame.setContent(channel.allocate());
                httpFrame.write(content, channel);
                channel.writeAndFlush(httpFrame);
            }
        };

        final NioEventLoopGroup group = new NioEventLoopGroup();
        group.setEnableMemoryPoolDirect(true);

        final var context = new ChannelAcceptor(group, 6006);
        context.addChannelEventListener(new LoggerChannelOpenListener());
        context.setIoEventHandle(ioEventHandle);
        context.addProtocolCodec(new HttpCodec());
        context.bind();
    }
}
